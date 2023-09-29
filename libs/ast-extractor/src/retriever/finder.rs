/**
 * finder.rs
 * Function to retrieve contract nodes from position
 * author: 0xSwapFeeder
 */
use syn_solidity::*;
use proc_macro2::LineColumn;
use syn::ExprLit;
use syn_solidity::kw::contract;
use syn_solidity::visit::{visit_expr_new, visit_variable_declaration};
use crate::retriever::finder::find_node::FoundNode;

mod find_node;

macro_rules! is_in_range {
    ($start:expr, $end:expr, $pos:expr) => {
        $pos.char >= $start.column && $pos.line <= $start.line
            && $pos.char <= $end.column && $pos.line <= $end.line
    };
}

pub struct Position {
    line: usize,
    char: usize,
}

impl Position {
    pub fn new(line: usize, char: usize) -> Self {
        Self {
            line,
            char,
        }
    }

}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 0,
            char: 0,
        }
    }
}

struct FinderVisitor {
    current_contract: Option<ItemContract>,
    current_function: Option<ItemFunction>,
    current_property: Option<VariableDefinition>,
    current_variable: Option<VariableDeclaration>,
    current_enum: Option<ItemEnum>,
    current_struct: Option<ItemStruct>,
    current_error: Option<ItemError>,
    current_event: Option<ItemEvent>,
    current_expr: Option<Expr>,
    current_stmt: Option<Stmt>,
    found: Option<FoundNode>,
    to_find: Position,
}


impl FinderVisitor {

    pub fn new(pos: Position) -> Self {
        Self {
            current_contract: None,
            current_function: None,
            current_property: None,
            current_variable: None,
            current_enum: None,
            current_struct: None,
            current_error: None,
            current_event: None,
            current_expr: None,
            current_stmt: None,
            found: None,
            to_find: pos,
        }
    }

    fn check_inheritance_matching(&mut self, contract: &ItemContract) -> bool {
        if let Some(inheritance) = &contract.inheritance {
            if is_in_range!(inheritance.span().start(), inheritance.span().end(), self.to_find) {
                for inherit in &inheritance.inheritance {
                    if is_in_range!(inherit.span().start(), inherit.span().end(), self.to_find) {
                        self.found = Some(FoundNode::ContractDefInheritance(contract.clone(), inherit.clone()));
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

impl<'ast> Visit<'ast> for FinderVisitor {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(stmt.span().start(), stmt.span().end(), self.to_find) {
            self.current_stmt = Some(stmt.clone());
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(expr.span().start(), expr.span().end(), self.to_find) {
            self.current_expr = Some(expr.clone());
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_expr(self, expr);
    }


    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(call.span().start(), call.span().end(), self.to_find) {
            self.current_expr = Some(Expr::Call(call.clone()));
            if !is_in_range!(call.args.span().start(), call.args.span().end(), self.to_find) {
                self.found = Some(FoundNode::FunctionUsageName(self.current_contract.clone().unwrap().clone(), self.current_function.clone().unwrap().clone(), call.clone()));
                return;
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_expr_call(self, call);
    }

    //TODO: Found Limitation: cannot check parameter list of a new expr
    // Therefore we can not goto or list_ref any variable used in a new expr
    fn visit_expr_new(&mut self, new: &'ast ExprNew) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(new.span().start(), new.span().end(), self.to_find) {
            self.current_expr = Some(Expr::New(new.clone()));
            self.found = Some(FoundNode::ContractInstantiation(self.current_contract.clone().unwrap().clone(), self.current_function.clone(), new.clone()));
        }
        if self.found.is_some() {
            return;
        }
        visit_expr_new(self, new);
    }

    fn visit_ident(&mut self, ident: &'ast SolIdent) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(ident.span().start(), ident.span().end(), self.to_find) {
            self.found = Some(FoundNode::VariableUsageName(self.current_contract.clone(), self.current_function.clone(), self.current_expr.clone().unwrap(), ident.clone()));
            return;
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_ident(self, ident);
    }

    fn visit_type(&mut self, ty: &'ast Type) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(ty.span().start(), ty.span().end(), self.to_find) {
            self.found = Some(FoundNode::TypeUsage(self.current_contract.clone(), self.current_function.clone(), self.current_expr.clone(), ty.clone()));
            return;
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_type(self, ty);
    }
    fn visit_variable_declaration(&mut self, var: &'ast VariableDeclaration) {
        if is_in_range!(var.span().start(), var.span().end(), self.to_find) {
            self.current_variable = Some(var.clone());
            if is_in_range!(var.name.span().start(), var.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::VariableDefName(self.current_contract.clone(), self.current_function.clone(), var.clone(), var.name.clone()));
                    return;
            }
        }
        if self.found.is_some() {
            return;
        }
        visit_variable_declaration(self, var);

    }

    fn visit_variable_definition(&mut self, var: &'ast VariableDefinition) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(var.span().start(), var.span().end(), self.to_find) {
            self.current_property = Some(var.clone());
            if is_in_range!(var.name.span().start(), var.name.span().end(), self.to_find) {
                if self.current_contract.is_none() {
                    self.found = Some(FoundNode::ConstantVariableDefName(var.clone(), var.name.clone()))
                } else {
                    self.found = Some(FoundNode::PropertyDefName(self.current_contract.clone().unwrap(),var.clone(), var.name.clone()));
                }
                return;
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_variable_definition(self, var);
    }

    fn visit_item_contract(&mut self, contract: &'ast ItemContract) {
        if self.found.is_some() {
            return;
        }
        self.current_contract = Some(contract.clone());
        if is_in_range!(contract.span().start(), contract.span().end(), self.to_find) {
            if is_in_range!(contract.name.span().start(), contract.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::ContractDefName(contract.clone()));
                return;
            }
            if self.check_inheritance_matching(contract) {
                return;
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_item_contract(self, contract);
    }

    fn visit_item_enum(&mut self, enumm: &'ast ItemEnum) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(enumm.span().start(), enumm.span().end(), self.to_find) {
            self.current_enum = Some(enumm.clone());
            if is_in_range!(enumm.name.span().start(), enumm.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::EnumDefName(self.current_contract.clone(),enumm.clone(), enumm.name.clone()));
                return;
            }
            for variant in &enumm.variants {
                if is_in_range!(variant.ident.span().start(), variant.ident.span().end(), self.to_find) {
                    self.found = Some(FoundNode::EnumDefValue(self.current_contract.clone(), enumm.clone(), variant.clone(), variant.ident.clone()));
                    return;
                }
            }
        }
        visit::visit_item_enum(self, enumm);
        if self.found.is_some() {
            return;
        }
    }

    fn visit_item_error(&mut self, error: &'ast ItemError) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(error.span().start(), error.span().end(), self.to_find) {
            self.current_error = Some(error.clone());
            if is_in_range!(error.name.span().start(), error.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::ErrorDefName(self.current_contract.clone(), error.clone(), error.name.clone()));
                return;
            }
            for param in &error.parameters {
                if is_in_range!(param.name.span().start(), param.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::ErrorDefParameter(self.current_contract.clone(), error.clone(), param.clone()));
                    return;
                }
            }
        }
        visit::visit_item_error(self, error);
        if self.found.is_some() {
            return;
        }
    }

    fn visit_item_event(&mut self, event: &'ast ItemEvent) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(event.span().start(), event.span().end(), self.to_find) {
            self.current_event = Some(event.clone());
            if is_in_range!(event.name.span().start(), event.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::EventDefName(self.current_contract.clone().unwrap().clone(), event.clone(), event.name.clone()));
                return;
            }
            for param in &event.parameters {
                if is_in_range!(param.name.span().start(), param.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::EventDefParameter(self.current_contract.clone().unwrap().clone(), event.clone(), param.clone()));
                    return;
                }
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_item_event(self, event);
    }

    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(function.span().start(), function.span().end(), self.to_find) {
            self.current_function = Some(function.clone());
            if is_in_range!(function.name.span().start(), function.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::FunctionDefName(self.current_contract.clone().unwrap(), function.clone()));
                return;
            }
            for param in &function.arguments {
                if is_in_range!(param.name.span().start(), param.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::FunctionDefParameterName(self.current_contract.clone().unwrap(), function.clone(), param.clone(), param.name.clone()));
                    return;
                }
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_item_function(self, function);
    }

    fn visit_item_struct(&mut self, strukt: &'ast ItemStruct) {
        if self.found.is_some() {
            return;
        }
        if is_in_range!(strukt.span().start(), strukt.span().end(), self.to_find) {
            self.current_struct = Some(strukt.clone());
            if is_in_range!(strukt.name.span().start(), strukt.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::StructDefName(self.current_contract.clone(), strukt.name.clone()));
                return;
            }
            for field in &strukt.fields {
                if is_in_range!(field.name.span().start(), field.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::StructDefPropertyName(self.current_contract.clone().unwrap().clone(), self.current_function.clone(), field.clone(), field.name.clone()));
                    return;
                }
            }
        }
        if self.found.is_some() {
            return;
        }
        visit::visit_item_struct(self, strukt);
    }

}


pub fn retrieve_node_from_position(ast: &File, pos: Position) -> Option<FoundNode> {
    let mut visitor = FinderVisitor::new(pos);
    visitor.visit_file(ast);
    visitor.found
}


#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_node_type_decl_string() {
        let source = String::from("pragma solidity ^0.8.0;\
        abstract contract One {
    uint storedData;
    function set(uint x) public {
        storedData = x;
        string test2;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_node_from_position(&ast, Position::new(5, 8));
        if let Some(node) = res {
                match &node {
                    FoundNode::TypeUsage(_,_,_,ty) => {
                        match ty {
                            Type::String(_) => {assert!(true)}
                            _ => {assert!(false)}
                        }
                    }
                    _ => {
                        assert!(false)
                    }
                }

            } else {
                assert!(false)
            }
    }

    #[test]
    fn test_retrieve_function_def_name() {
        let source = String::from("pragma solidity ^0.8.0;\
        abstract contract One {
    uint storedData;
    function set(uint x) public {
        storedData = x;
        string test2;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = parse2(tokens).unwrap();
        let res = retrieve_node_from_position(&ast, Position::new(3, 14));
        if let Some(node) = res {
            match &node {
                FoundNode::FunctionDefName(_,f) => {
                    if let Some(name) = &f.name {
                        assert_eq!(name.to_string(), "set");
                    } else {
                        assert!(false)
                    }
                }
                _ => {
                    assert!(false)
                }
            }

        } else {
            assert!(false)
        }
    }
}
