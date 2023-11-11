use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// global
pub const RULE_ID: &str = "named-parameters-mapping";

// specific
const DEFAULT_SEVERITY: Severity = Severity::WARNING;

pub struct NamedParametersMapping {
    data: RuleEntry,
}

pub struct MappingsVisitor {
    mappings: Vec<TypeMapping>,
}

impl MappingsVisitor {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for MappingsVisitor {
    fn visit_type(&mut self, t: &Type) {
        if let Type::Mapping(ty) = t {
            self.mappings.push(ty.clone());
        }
        visit::visit_type(self, t)
    }
}

impl NamedParametersMapping {
    fn create_diag(
        &self,
        location: (LineColumn, LineColumn),
        message: &str,
        file: &SolidFile,
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
            message: message.to_string(),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for NamedParametersMapping {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = MappingsVisitor::new();
        for contract in
            osmium_libs_solidity_ast_extractor::retriever::retrieve_contract_nodes(&file.data)
        {
            visitor.visit_item_contract(&contract);
        }

        for mapping in visitor.mappings.iter() {
            if mapping.key_name.is_none() {
                let span = mapping.key.span();
                res.push(self.create_diag(
                    (span.start(), span.end()),
                    format!("{} parameter is not named", mapping.key).as_str(),
                    file,
                ));
            }
            if mapping.value_name.is_none() {
                let span = mapping.value.span();
                res.push(self.create_diag(
                    (span.start(), span.end()),
                    format!("{} parameter is not named", mapping.value).as_str(),
                    file,
                ));
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

impl NamedParametersMapping {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = NamedParametersMapping { data };
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
