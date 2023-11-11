use osmium_libs_solidity_ast_extractor::Visibility::{Internal, Private};
use osmium_libs_solidity_ast_extractor::{Item, LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "private-vars-leading-underscore";

// specific
const MESSAGE_PRIVATE: &str = "Private and internal variables must start with a single underscore";
const MESSAGE_PUBLIC: &str =
    "Only private and internal variables must start with a single underscore";
const DEFAULT_STRICT: bool = false;
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct PrivateVarsLeadingUnderscore {
    data: RuleEntry,
    strict: bool,
}

impl PrivateVarsLeadingUnderscore {
    fn create_diag(
        &self,
        location: (LineColumn, LineColumn),
        file: &SolidFile,
        message: String,
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
            message,
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for PrivateVarsLeadingUnderscore {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            let functions =
                osmium_libs_solidity_ast_extractor::retriever::retrieve_functions_nodes(&contract);

            for function in functions {
                if self.strict {
                    for argument in function.arguments {
                        if let Some(name) = argument.name {
                            let leading_underscore = name.as_string().starts_with('_');

                            if !leading_underscore {
                                let span = name.span();
                                res.push(self.create_diag(
                                    (span.start(), span.end()),
                                    file,
                                    MESSAGE_PRIVATE.to_string(),
                                ));
                            }
                        }
                    }

                    if let Some(returns) = function.returns {
                        for return_arg in returns.returns {
                            if let Some(name) = return_arg.name {
                                let leading_underscore = name.as_string().starts_with('_');

                                if !leading_underscore {
                                    let span = name.span();
                                    res.push(self.create_diag(
                                        (span.start(), span.end()),
                                        file,
                                        MESSAGE_PRIVATE.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }

                let is_private = match function.attributes.visibility() {
                    Some(val) => matches!(val, Private(_) | Internal(_)),
                    None => true,
                };

                if let Some(name) = function.name {
                    let leading_underscore = name.as_string().starts_with('_');

                    if !leading_underscore && is_private {
                        let span = name.span();
                        res.push(self.create_diag(
                            (span.start(), span.end()),
                            file,
                            MESSAGE_PRIVATE.to_string(),
                        ));
                    }
                    if leading_underscore && !is_private {
                        let span = name.span();
                        res.push(self.create_diag(
                            (span.start(), span.end()),
                            file,
                            MESSAGE_PUBLIC.to_string(),
                        ));
                    }
                }
            }

            for node_var in contract.body.iter() {
                if let Item::Variable(var) = node_var {
                    let is_private = match var.attributes.visibility() {
                        Some(val) => matches!(val, Private(_) | Internal(_)),
                        None => true,
                    };

                    let leading_underscore = var.name.as_string().starts_with('_');

                    if !leading_underscore && is_private {
                        let span = var.name.span();
                        res.push(self.create_diag(
                            (span.start(), span.end()),
                            file,
                            MESSAGE_PRIVATE.to_string(),
                        ));
                    }
                    if leading_underscore && !is_private {
                        let span = var.name.span();
                        res.push(self.create_diag(
                            (span.start(), span.end()),
                            file,
                            MESSAGE_PUBLIC.to_string(),
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

impl PrivateVarsLeadingUnderscore {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut strict = DEFAULT_STRICT;

        if let Some(data) = &data.data {
            if !data["strict"].is_null() && data["strict"].as_bool().is_some() {
                strict = data["strict"].as_bool().unwrap();
            } else {
                eprintln!("{} rule : bad config data", RULE_ID);
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = PrivateVarsLeadingUnderscore { strict, data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(serde_json::json!({
                "strict": DEFAULT_STRICT,
            })),
        }
    }
}
