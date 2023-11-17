use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::parse_rules;
use crate::rules::rule_impl::parse_rules_content;
use crate::rules::types::*;
use crate::types::*;
use std::fs;

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
        }
    }

    pub fn new_fileless() -> Self {
        let default_rules = create_default_rules();
        let mut linter = SolidLinter {
            files: Vec::new(),
            rule_factory: RuleFactory::default(),
            rules: Vec::new(),
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

    pub fn initialize_rules_content(&mut self, rules_config: &str) -> Result<(), SolidHunterError> {
        let res = parse_rules_content(rules_config)?;
        for rule in res.rules {
            self.rules.push(self.rule_factory.create_rule(rule));
        }
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

    pub fn parse_file(&mut self, filepath: &str) -> LintResult {
        let content = fs::read_to_string(filepath)?;
        self.parse_content(filepath, content.as_str())
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
                result.push(self.parse_file(&entry.into_os_string().into_string().unwrap()));
            }
        }
        result
    }
    pub fn parse_path(&mut self, path: &str) -> Vec<LintResult> {
        if Path::new(&path).is_file() {
            vec![self.parse_file(path)]
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
