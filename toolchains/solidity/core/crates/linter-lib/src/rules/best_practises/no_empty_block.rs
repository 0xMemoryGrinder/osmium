use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::{
    retriever::{retrieve_block_nodes, retrieve_contract_nodes},
    Spanned,
};

// global
pub const RULE_ID: &str = "no-empty-block";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Code contains empty blocks";

pub struct NoEmptyBlock {
    data: RuleEntry,
}

impl RuleType for NoEmptyBlock {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let _reports = check_empty_block(_file);
        for report in _reports.iter().flatten() {
            res.push(LintDiag {
                id: RULE_ID.to_string(),
                severity: self.data.severity,
                range: report.clone(),
                code: None,
                source: None,
                message: DEFAULT_MESSAGE.to_string(),
                uri: _file.path.clone(),
            });
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

fn check_empty_block(file: &SolidFile) -> Vec<Option<Range>> {
    let mut res: Vec<Option<Range>> = Vec::new();

    let contracts = retrieve_contract_nodes(&file.data);
    for contract in contracts.iter() {
        if contract.body.is_empty() {
            res.push(Some(Range {
                start: Position {
                    line: contract.span().start().line,
                    character: contract.span().start().column + 1,
                },
                end: Position {
                    line: contract.span().end().line,
                    character: contract.span().end().column,
                },
            }));
        }
    }

    let blocks = retrieve_block_nodes(&file.data);
    for block in blocks.iter() {
        if block.stmts.is_empty() {
            res.push(Some(Range {
                start: Position {
                    line: block.span().start().line,
                    character: block.span().start().column + 1,
                },
                end: Position {
                    line: block.span().end().line,
                    character: block.span().end().column,
                },
            }));
        }
    }
    res
}

impl NoEmptyBlock {
    pub fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NoEmptyBlock { data };
        Box::new(rule)
    }

    pub fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
