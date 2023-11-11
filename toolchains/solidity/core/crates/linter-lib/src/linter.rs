use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::parse_rules;
use crate::rules::types::*;
use crate::types::*;
use std::fs;

use crate::ignore::get_excluded_files;
use glob::glob;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SolidFile {
    pub data: osmium_libs_solidity_ast_extractor::File,
    pub path: String,
    pub content: String,
}

pub struct SolidLinter {
    files: Vec<SolidFile>,
    rule_factory: RuleFactory,
    rules: Vec<Box<dyn RuleType>>,
    excluded_files: Vec<String>,
}

impl Default for SolidLinter {
    fn default() -> Self {
        SolidLinter::new()
    }
}

impl SolidLinter {
    pub fn new() -> Self {
        SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: vec![],
            excluded_files: Vec::new(),
        }
    }

    pub fn new_fileless() -> Self {
        let default_rules = create_default_rules();
        let mut linter = SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: Vec::new(),
            excluded_files: Vec::new(),
        };

        for rule in default_rules {
            linter.rules.push(linter.rule_factory.create_rule(rule));
        }

        linter
    }

    pub fn initialize_rules(&mut self, rules_config: &str) -> Result<(), SolidHunterError> {
        let res = parse_rules(rules_config)?;
        for rule in res.rules {
            self.rules.push(self.rule_factory.create_rule(rule));
        }
        Ok(())
    }

    pub fn get_documentation(&self) -> Vec<RuleDocumentation> {
        let mut res = Vec::new();
        for rule in &self.rules {
            res.push(rule.get_documentation())
        }
        res
    pub fn initialize_excluded_files(
        &mut self,
        excluded_filepaths: Option<&Vec<String>>,
        filepaths: &Vec<String>,
    ) -> Result<(), SolidHunterError> {
        if let Some(excluded) = excluded_filepaths {
            for path in excluded {
                self.excluded_files.push(path.clone())
            }
        }
        self.excluded_files
            .append(&mut get_excluded_files(filepaths)?);

        Ok(())
    }

    fn _file_exists(&self, path: &str) -> bool {
        for file in &self.files {
            if file.path == path {
                return true;
            }
        }
        false
    }

    fn _add_file(
        &mut self,
        path: &str,
        ast: osmium_libs_solidity_ast_extractor::File,
        content: &str,
    ) {
        if self._file_exists(path) {
            for file in &mut self.files {
                if file.path == path {
                    file.data = ast.clone();
                    file.content = String::from(content);
                }
            }
        } else {
            let file = SolidFile {
                data: ast,
                path: String::from(path),
                content: String::from(content),
            };
            self.files.push(file);
        }
    }

    pub fn parse_file(&mut self, filepath: String) -> LintResult {
        let content = fs::read_to_string(filepath.clone())?;
        if self.excluded_files.contains(&filepath) {
            return Ok(FileDiags::new(content, Vec::new()));
        }
        self.parse_content(&filepath, content.as_str())
    }

    pub fn parse_content(&mut self, filepath: &str, content: &str) -> LintResult {
        let res = osmium_libs_solidity_ast_extractor::extract::extract_ast_from_content(content)?;

        self._add_file(filepath, res, content);
        let mut res: Vec<LintDiag> = Vec::new();

        for rule in &self.rules {
            let mut diags = rule.diagnose(&self.files[self.files.len() - 1], &self.files);
            res.append(&mut diags);
        }
        Ok(FileDiags::new(content.to_string(), res))
    }

    pub fn parse_folder(&mut self, folder: &str) -> Vec<LintResult> {
        let mut result: Vec<LintResult> = Vec::new();
        if let Ok(entries) = glob(&(folder.to_owned() + "/**/*.sol")) {
            for entry in entries.flatten() {
                result.push(self.parse_file(entry.into_os_string().into_string().unwrap()));
            }
        }
        result
    }
    pub fn parse_path(&mut self, path: &str) -> Vec<LintResult> {
        if Path::new(&path).is_file() {
            vec![self.parse_file(path.to_string())]
        } else {
            self.parse_folder(path)
        }
    }

    pub fn delete_file(&mut self, path: &str) {
        loop {
            let idx = self.files.iter().position(|x| x.path == path);
            match idx {
                Some(idx) => {
                    self.files.remove(idx);
                }
                None => {
                    break;
                }
            }
        }
    }
}
