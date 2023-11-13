use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::factory::RuleFactory;
use crate::rules::rule_impl::parse_rules;
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

    fn _check_is_in_disable_range(
        &self,
        diag: &LintDiag,
        ignore_states: &Vec<(usize, Ignore, Vec<&str>)>,
    ) -> bool {
        let mut rules_occurences: Vec<(&str, i32)> = Vec::new();
        let ignore_states: Vec<(usize, Ignore, Vec<&str>)> = ignore_states
            .iter()
            .filter(|(line, _, _)| *line <= diag.range.start.line)
            .map(|(line, ignore, rules)| (*line, *ignore, rules.to_vec()))
            .map(|(line, ignore, rules)| {
                if rules.is_empty() {
                    (line, ignore, vec![""]) // empty rule means all rules
                } else {
                    (line, ignore, rules)
                }
            })
            .collect::<Vec<(usize, Ignore, Vec<&str>)>>();

        for (_, ignore, rules) in ignore_states {
            match ignore {
                Ignore::Disable => {
                    for rule in rules {
                        let mut found = false;
                        for (rule_id, occurences) in &mut rules_occurences {
                            if *rule_id == rule {
                                *occurences += 1;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            rules_occurences.push((rule, 1));
                        }
                    }
                }
                Ignore::Enable => {
                    for rule in rules {
                        for (rule_id, occurences) in &mut rules_occurences {
                            if *rule_id == rule {
                                *occurences -= 1;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let disabled_rules = rules_occurences
            .iter()
            .filter(|(_, occurences)| *occurences > 0)
            .map(|(rule, _)| *rule)
            .collect::<Vec<&str>>();

        for rule in disabled_rules {
            if rule.is_empty() {
                return true;
            } else if rule == diag.id.as_str() {
                return true;
            }
        }

        false
    }

    fn _check_is_diag_ignored(&self, diag: &LintDiag, file: &SolidFile) -> bool {
        let comments = file
            .content
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                for ignore in Ignore::iter() {
                    let ignore_str = ignore.to_string();
                    if line.contains(&ignore_str) {
                        return Some((i + 1, ignore, line.split(&ignore_str).nth(1)));
                    }
                }
                None
            })
            .collect::<Vec<(usize, Ignore, Option<&str>)>>();

        let mut ignore_states: Vec<(usize, Ignore, Vec<&str>)> = Vec::new();

        for (line, ignore, rule_ids_str) in comments {
            let rules_ids = rule_ids_str
                .map(|s| {
                    s.split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>()
                })
                .filter(|s| !s.is_empty());

            match ignore {
                Ignore::Enable => {
                    ignore_states.push((line, ignore, rules_ids.clone().unwrap_or(Vec::new())));
                }
                Ignore::Disable => {
                    ignore_states.push((line, ignore, rules_ids.clone().unwrap_or(Vec::new())));
                }
                _ => {}
            }

            if diag.range.start.line == line + if ignore == Ignore::SameLine { 0 } else { 1 } {
                match rules_ids {
                    // If rules are specified, we ignore only the specified rules
                    Some(rules_ids) => {
                        if rules_ids.contains(&diag.id.as_str()) {
                            return true;
                        }
                    }
                    // If no rules are specified, we ignore all rules
                    None => {
                        return true;
                    }
                }
            }
        }

        if self._check_is_in_disable_range(diag, &ignore_states) {
            return true;
        }

        false
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
            for diag in &mut diags {
                if !self._check_is_diag_ignored(diag, &self.files[self.files.len() - 1]) {
                    res.push(diag.clone());
                }
            }
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
