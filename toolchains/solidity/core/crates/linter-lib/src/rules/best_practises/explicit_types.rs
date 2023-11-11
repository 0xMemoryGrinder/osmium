use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use osmium_libs_solidity_ast_extractor::*;

// TODO test output

// global
pub const RULE_ID: &str = "explicit-types";

// specific
const DEFAULT_RULE: &str = "explicit";
const DEFAULT_SEVERITY: Severity = Severity::WARNING;
const EXPLICIT_TYPES: &[&str] = &[
    "uint256", "int256", "uint8", "int8", "uint16", "int16", "uint32", "int32", "uint64", "int64",
    "uint128", "int128",
];
const IMPLICIT_TYPES: &[&str] = &["uint", "int"];

pub struct ExplicitTypes {
    rule: String,
    data: RuleEntry,
}

pub struct ExplicitTypesVisitor {
    explicit: bool,
    defs: Vec<VariableDefinition>,
    decls: Vec<VariableDeclaration>,
    types: Vec<Type>,
}

impl<'ast> Visit<'ast> for ExplicitTypesVisitor {
    fn visit_variable_definition(&mut self, var: &'ast VariableDefinition) {
        if let Some((_, expr)) = &var.initializer {
            visit::visit_expr(self, expr);
        }
        if self.is_type_match(&var.ty) {
            self.defs.push(var.clone())
        }
    }

    fn visit_variable_declaration(&mut self, var: &'ast VariableDeclaration) {
        if self.is_type_match(&var.ty) {
            self.decls.push(var.clone())
        }
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        if self.is_type_match(ty) {
            self.types.push(ty.clone());
        }
    }
}

impl ExplicitTypesVisitor {
    fn is_type_match(&self, ty: &Type) -> bool {
        if self.explicit {
            IMPLICIT_TYPES.iter().any(|typ| ty.to_string() == *typ)
        } else {
            EXPLICIT_TYPES.iter().any(|typ| ty.to_string() == *typ)
        }
    }
}

impl ExplicitTypes {
    fn create_diag(&self, file: &SolidFile, ty: Box<dyn Spanned>, var: Option<String>) -> LintDiag {
        LintDiag {
            range: Range {
                start: Position {
                    line: ty.span().start().line,
                    character: ty.span().start().column,
                },
                end: Position {
                    line: ty.span().end().line,
                    character: ty.span().end().column,
                },
            },
            id: RULE_ID.to_string(),
            message: format!(
                "Rule is set with {} type [var/s: {}]",
                self.rule,
                var.unwrap_or("\"\"".to_string())
            ),
            severity: self.data.severity,
            code: None,
            source: None,
            uri: file.path.clone(),
        }
    }
}

impl RuleType for ExplicitTypes {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = ExplicitTypesVisitor {
            explicit: self.rule == "explicit",
            defs: vec![],
            decls: vec![],
            types: vec![],
        };
        visitor.visit_file(&file.data);
        for def in visitor.defs {
            res.push(self.create_diag(file, Box::new(def.ty), Some(def.name.0.to_string())));
        }
        for decl in visitor.decls {
            let name = match decl.name {
                Some(ident) => Some(ident.0.to_string()),
                _ => None,
            };
            res.push(self.create_diag(file, Box::new(decl.ty), name));
        }
        for ty in visitor.types {
            res.push(self.create_diag(file, Box::new(ty), None));
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

impl ExplicitTypes {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut value = DEFAULT_RULE.to_string();

        if let Some(data) = &data.data {
            let parsed: Result<String, serde_json::Error> = serde_json::from_value(data.clone());
            match parsed {
                Ok(val) => value = val,
                Err(_) => {
                    eprintln!("{} rule : bad config data", RULE_ID);
                }
            }
        } else {
            eprintln!("{} rule : bad config data", RULE_ID);
        }
        let rule = ExplicitTypes { rule: value, data };
        Box::new(rule)
    }
    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: DEFAULT_SEVERITY,
            data: Some(DEFAULT_RULE.into()),
        }
    }
}
