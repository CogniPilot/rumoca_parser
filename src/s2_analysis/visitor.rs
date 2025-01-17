use crate::s1_parser::ast;

pub trait Visitor {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // File Level Nodes

    fn enter_stored_definition(&mut self, _def: &mut ast::StoredDefinition) {}
    fn exit_stored_definition(&mut self, _def: &mut ast::StoredDefinition) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Class Level Node

    fn enter_class_definition(&mut self, _def: &mut ast::ClassDefinition) {}
    fn exit_class_definition(&mut self, _def: &mut ast::ClassDefinition) {}

    fn enter_class_specifier(&mut self, _def: &mut ast::ClassSpecifier) {}
    fn exit_class_specifier(&mut self, _def: &mut ast::ClassSpecifier) {}

    fn enter_composition_part(&mut self, _def: &mut ast::CompositionPart) {}
    fn exit_composition_part(&mut self, _def: &mut ast::CompositionPart) {}

    fn enter_element(&mut self, _def: &mut ast::Element) {}
    fn exit_element(&mut self, _def: &mut ast::Element) {}

    fn enter_component_declaration(&mut self, _def: &mut ast::ComponentDeclaration) {}
    fn exit_component_declaration(&mut self, _def: &mut ast::ComponentDeclaration) {}

    fn enter_component_declaration1(&mut self, _def: &mut ast::ComponentDeclaration1) {}
    fn exit_component_declaration1(&mut self, _def: &mut ast::ComponentDeclaration1) {}

    fn enter_class_prefixes(&mut self, _def: &mut ast::ClassPrefixes) {}
    fn exit_class_prefixes(&mut self, _def: &mut ast::ClassPrefixes) {}

    fn enter_component_clause(&mut self, _def: &mut ast::ComponentClause) {}
    fn exit_component_clause(&mut self, _def: &mut ast::ComponentClause) {}

    fn enter_component_clause1(&mut self, _def: &mut ast::ComponentClause1) {}
    fn exit_component_clause1(&mut self, _def: &mut ast::ComponentClause1) {}

    fn enter_declaration(&mut self, _def: &mut ast::Declaration) {}
    fn exit_declaration(&mut self, _def: &mut ast::Declaration) {}

    fn enter_type_specifier(&mut self, _def: &mut ast::TypeSpecifier) {}
    fn exit_type_specifier(&mut self, _def: &mut ast::TypeSpecifier) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Equation Node

    fn enter_equation(&mut self, _def: &mut ast::Equation) {}
    fn exit_equation(&mut self, _def: &mut ast::Equation) {}

    fn enter_if_equation_block(&mut self, _def: &mut ast::IfEquationBlock) {}
    fn exit_if_equation_block(&mut self, _def: &mut ast::IfEquationBlock) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Statement Nodes

    fn enter_statement(&mut self, _def: &mut ast::Statement) {}
    fn exit_statement(&mut self, _def: &mut ast::Statement) {}

    fn enter_if_statement_block(&mut self, _def: &mut ast::IfStatementBlock) {}
    fn exit_if_statement_block(&mut self, _def: &mut ast::IfStatementBlock) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Expression Nodes

    fn enter_expression(&mut self, _def: &mut ast::Expression) {}
    fn exit_expression(&mut self, _def: &mut ast::Expression) {}

    fn enter_if_expression_block(&mut self, _def: &mut ast::IfExpressionBlock) {}
    fn exit_if_expression_block(&mut self, _def: &mut ast::IfExpressionBlock) {}

    fn enter_component_reference(&mut self, _def: &mut ast::ComponentReference) {}
    fn exit_component_reference(&mut self, _def: &mut ast::ComponentReference) {}

    fn enter_ref_part(&mut self, _def: &mut ast::RefPart) {}
    fn exit_ref_part(&mut self, _def: &mut ast::RefPart) {}

    fn enter_array_subscripts(&mut self, _def: &mut ast::ArraySubscripts) {}
    fn exit_array_subscripts(&mut self, _def: &mut ast::ArraySubscripts) {}

    fn enter_subscript(&mut self, _def: &mut ast::Subscript) {}
    fn exit_subscript(&mut self, _def: &mut ast::Subscript) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Modification Nodes

    fn enter_argument(&mut self, _def: &mut ast::Argument) {}
    fn exit_argument(&mut self, _def: &mut ast::Argument) {}

    fn enter_modification(&mut self, _def: &mut ast::Modification) {}
    fn exit_modification(&mut self, _def: &mut ast::Modification) {}

    fn enter_mod_expr(&mut self, _def: &mut ast::ModExpr) {}
    fn exit_mod_expr(&mut self, _def: &mut ast::ModExpr) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Common Nodes

    fn enter_description(&mut self, _def: &mut ast::Description) {}
    fn exit_description(&mut self, _def: &mut ast::Description) {}

    fn enter_type_prefix(&mut self, _def: &mut ast::TypePrefix) {}
    fn exit_type_prefix(&mut self, _def: &mut ast::TypePrefix) {}

    fn enter_for_index(&mut self, _def: &mut ast::ForIndex) {}
    fn exit_for_index(&mut self, _def: &mut ast::ForIndex) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Simple Terminal Nodes

    fn enter_span(&mut self, _def: &mut ast::Span) {}
    fn exit_span(&mut self, _def: &mut ast::Span) {}

    fn enter_element_flags(&mut self, _def: &mut ast::ElementFlags) {}
    fn exit_element_flags(&mut self, _def: &mut ast::ElementFlags) {}

    fn enter_causality(&mut self, _def: &mut ast::Causality) {}
    fn exit_causality(&mut self, _def: &mut ast::Causality) {}

    fn enter_variability(&mut self, _def: &mut ast::Variability) {}
    fn exit_variability(&mut self, _def: &mut ast::Variability) {}

    fn enter_visibility(&mut self, _def: &mut ast::Visibility) {}
    fn exit_visibility(&mut self, _def: &mut ast::Visibility) {}

    fn enter_connection(&mut self, _def: &mut ast::Connection) {}
    fn exit_connection(&mut self, _def: &mut ast::Connection) {}

    fn enter_unary_op(&mut self, _def: &mut ast::UnaryOp) {}
    fn exit_unary_op(&mut self, _def: &mut ast::UnaryOp) {}

    fn enter_binary_op(&mut self, _def: &mut ast::BinaryOp) {}
    fn exit_binary_op(&mut self, _def: &mut ast::BinaryOp) {}

    fn enter_class_type(&mut self, _def: &mut ast::ClassType) {}
    fn exit_class_type(&mut self, _def: &mut ast::ClassType) {}

    fn enter_name(&mut self, _def: &mut ast::Name) {}
    fn exit_name(&mut self, _def: &mut ast::Name) {}
}
