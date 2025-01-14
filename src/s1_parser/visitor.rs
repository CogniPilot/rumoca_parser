use crate::s1_parser::ast;

pub trait Visitor<'a> {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

    #[allow(unused_variables)]
    fn enter_expression(&mut self, _expr: &'a ast::Expression) {}
    fn exit_expression(&mut self, _expr: &'a ast::Expression) {}

    fn enter_element(&mut self, _elem: &'a ast::Element) {}
    fn exit_element(&mut self, _elem: &'a ast::Element) {}

    fn enter_class_definition(&mut self, _class: &'a ast::ClassDefinition) {}
    fn exit_class_definition(&mut self, _class: &'a ast::ClassDefinition) {}

    fn enter_equation(&mut self, _eq: &'a ast::Equation) {}
    fn exit_equation(&mut self, _eq: &'a ast::Equation) {}

    fn enter_statement(&mut self, _stmt: &'a ast::Statement) {}
    fn exit_statement(&mut self, _stmt: &'a ast::Statement) {}

    fn enter_stored_definition(&mut self, _def: &'a ast::StoredDefinition) {}
    fn exit_stored_definition(&mut self, _def: &'a ast::StoredDefinition) {}

    fn enter_component_reference(&mut self, _comp: &'a ast::ComponentReference) {}
    fn exit_component_reference(&mut self, _comp: &'a ast::ComponentReference) {}
}
