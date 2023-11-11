use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "foundry-test-functions";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_SKIP_FUNCTIONS: &[&str] = &["setUp"];

pub struct FoundryTestFunctions {
    data: RuleEntry,
    excluded: Vec<String>,
}

impl FoundryTestFunctions {
    fn create_diag(
        &self,
        location: (LineColumn, LineColumn),
        file: &SolidFile,
        name: &str,
    ) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: Range {
                start: Position {
                    line: location.0.line,
                    character: location.0.column,
                },
                end: Position {
                    line: location.1.line,
                    character: location.1.column,
                },
            },
            message: format!(
                "Function {}() must match Foundry test naming convention",
                name
            ),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for FoundryTestFunctions {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        if !file.path.ends_with(".t.sol") {
            return vec![];
        }
        let mut res = Vec::new();
        let re = regex::Regex::new(r"^test(Fork)?(Fuzz)?(Fail)?(_)?(Revert(If_|When_){1})?\w{1,}$")
            .unwrap();
        let contracts =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in
                osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes(&contract)
            {
                let visibility = function.attributes.iter().find(|attr| {
                    matches!(
                        attr,
                        osmium_libs_solidity_ast_extractor::FunctionAttribute::Visibility(_)
                    )
                });
                let visibility = match visibility {
                    Some(osmium_libs_solidity_ast_extractor::FunctionAttribute::Visibility(
                        visibility,
                    )) => visibility,
                    _ => continue,
                };

                if !matches!(
                    visibility,
                    osmium_libs_solidity_ast_extractor::Visibility::Public(_)
                ) && !matches!(
                    visibility,
                    osmium_libs_solidity_ast_extractor::Visibility::External(_)
                ) {
                    continue;
                }
                if let Some(name) = function.name {
                    if !re.is_match(&name.as_string()) && !self.excluded.contains(&name.as_string())
                    {
                        let span = name.span();
                        res.push(self.create_diag(
                            (span.start(), span.end()),
                            file,
                            &name.as_string(),
                        ));
                    }
                }
            }
        }
        res
    }

    fn get_documentation(&self) -> RuleDocumentation {
        RuleDocumentation {
            id: RULE_ID.to_string(),
            description: "".to_string(),
            category: "".to_string(),
            options: vec![],
            examples: Examples {
                good: vec![],
                bad: vec![],
            },
        }
    }
}

impl FoundryTestFunctions {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut excluded: Vec<String> = Vec::new();

        if let Some(data) = &data.data {
            let parsed: Result<Vec<String>, serde_json::Error> =
                serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => excluded = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = FoundryTestFunctions { excluded, data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_SKIP_FUNCTIONS.into()),
        }
    }
}
