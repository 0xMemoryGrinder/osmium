use osmium_libs_solidity_ast_extractor::{LineColumn, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// global
pub const RULE_ID: &str = "one-contract-per-file";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const DEFAULT_MESSAGE: &str = "Found more than one contract per file";

pub struct OneContractPerFile {
    data: RuleEntry,
}

impl OneContractPerFile {
    fn create_diag(&self, location: (LineColumn, LineColumn), file: &SolidFile) -> LintDiag {
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
            message: DEFAULT_MESSAGE.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for OneContractPerFile {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts =
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data);
        let contract_count = contracts.len();

        if contract_count > 1 {
            for contract in &contracts[1..] {
                let span = contract.name.span();
                res.push(self.create_diag((span.start(), span.end()), file));
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

impl OneContractPerFile {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = OneContractPerFile { data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: None,
        }
    }
}
