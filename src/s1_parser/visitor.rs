use crate::s1_parser::ast;

pub trait Visitor {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

    fn enter_expression(&mut self, _expr: &ast::Expression) {}
    fn exit_expression(&mut self, _expr: &ast::Expression) {}

    fn enter_element(&mut self, _expr: &ast::Element) {}
    fn exit_element(&mut self, _expr: &ast::Element) {}

    fn enter_class_definition(&mut self, _class: &ast::ClassDefinition) {}
    fn exit_class_definition(&mut self, _class: &ast::ClassDefinition) {}

    fn enter_equation(&mut self, _eq: &ast::Equation) {}
    fn exit_equation(&mut self, _eq: &ast::Equation) {}

    fn enter_statement(&mut self, _stmt: &ast::Statement) {}
    fn exit_statement(&mut self, _stmt: &ast::Statement) {}

    fn enter_stored_definition(&mut self, _def: &ast::StoredDefinition) {}
    fn exit_stored_definition(&mut self, _def: &ast::StoredDefinition) {}

    fn enter_component_reference(&mut self, _def: &ast::ComponentReference) {}
    fn exit_component_reference(&mut self, _def: &ast::ComponentReference) {}
}
