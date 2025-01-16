use crate::s1_parser::ast;

pub trait Visitor<'a> {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // File Level Nodes

    fn enter_stored_definition(&mut self, _def: &'a ast::StoredDefinition) {}
    fn exit_stored_definition(&mut self, _def: &'a ast::StoredDefinition) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Class Level Node

    fn enter_class_definition(&mut self, _def: &'a ast::ClassDefinition) {}
    fn exit_class_definition(&mut self, _def: &'a ast::ClassDefinition) {}

    fn enter_class_specifier(&mut self, _def: &'a ast::ClassSpecifier) {}
    fn exit_class_specifier(&mut self, _def: &'a ast::ClassSpecifier) {}

    fn enter_composition_part(&mut self, _def: &'a ast::CompositionPart) {}
    fn exit_composition_part(&mut self, _def: &'a ast::CompositionPart) {}

    fn enter_element(&mut self, _def: &'a ast::Element) {}
    fn exit_element(&mut self, _def: &'a ast::Element) {}

    fn enter_component_declaration(&mut self, _def: &'a ast::ComponentDeclaration) {}
    fn exit_component_declaration(&mut self, _def: &'a ast::ComponentDeclaration) {}

    fn enter_component_declaration1(&mut self, _def: &'a ast::ComponentDeclaration1) {}
    fn exit_component_declaration1(&mut self, _def: &'a ast::ComponentDeclaration1) {}

    fn enter_class_prefixes(&mut self, _def: &'a ast::ClassPrefixes) {}
    fn exit_class_prefixes(&mut self, _def: &'a ast::ClassPrefixes) {}

    fn enter_component_clause(&mut self, _def: &'a ast::ComponentClause) {}
    fn exit_component_clause(&mut self, _def: &'a ast::ComponentClause) {}

    fn enter_component_clause1(&mut self, _def: &'a ast::ComponentClause1) {}
    fn exit_component_clause1(&mut self, _def: &'a ast::ComponentClause1) {}

    fn enter_declaration(&mut self, _def: &'a ast::Declaration) {}
    fn exit_declaration(&mut self, _def: &'a ast::Declaration) {}

    fn enter_type_specifier(&mut self, _def: &'a ast::TypeSpecifier) {}
    fn exit_type_specifier(&mut self, _def: &'a ast::TypeSpecifier) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Equation Node

    fn enter_equation(&mut self, _def: &'a ast::Equation) {}
    fn exit_equation(&mut self, _def: &'a ast::Equation) {}

    fn enter_if_equation_block(&mut self, _def: &'a ast::IfEquationBlock) {}
    fn exit_if_equation_block(&mut self, _def: &'a ast::IfEquationBlock) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Statement Nodes

    fn enter_statement(&mut self, _def: &'a ast::Statement) {}
    fn exit_statement(&mut self, _def: &'a ast::Statement) {}

    fn enter_if_statement_block(&mut self, _def: &'a ast::IfStatementBlock) {}
    fn exit_if_statement_block(&mut self, _def: &'a ast::IfStatementBlock) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Expression Nodes

    fn enter_expression(&mut self, _def: &'a ast::Expression) {}
    fn exit_expression(&mut self, _def: &'a ast::Expression) {}

    fn enter_if_expression_block(&mut self, _def: &'a ast::IfExpressionBlock) {}
    fn exit_if_expression_block(&mut self, _def: &'a ast::IfExpressionBlock) {}

    fn enter_component_reference(&mut self, _def: &'a ast::ComponentReference) {}
    fn exit_component_reference(&mut self, _def: &'a ast::ComponentReference) {}

    fn enter_ref_part(&mut self, _def: &'a ast::RefPart) {}
    fn exit_ref_part(&mut self, _def: &'a ast::RefPart) {}

    fn enter_array_subscripts(&mut self, _def: &'a ast::ArraySubscripts) {}
    fn exit_array_subscripts(&mut self, _def: &'a ast::ArraySubscripts) {}

    fn enter_subscript(&mut self, _def: &'a ast::Subscript) {}
    fn exit_subscript(&mut self, _def: &'a ast::Subscript) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Modification Nodes

    fn enter_argument(&mut self, _def: &'a ast::Argument) {}
    fn exit_argument(&mut self, _def: &'a ast::Argument) {}

    fn enter_modification(&mut self, _def: &'a ast::Modification) {}
    fn exit_modification(&mut self, _def: &'a ast::Modification) {}

    fn enter_mod_expr(&mut self, _def: &'a ast::ModExpr) {}
    fn exit_mod_expr(&mut self, _def: &'a ast::ModExpr) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Common Nodes

    fn enter_description(&mut self, _def: &'a ast::Description) {}
    fn exit_description(&mut self, _def: &'a ast::Description) {}

    fn enter_type_prefix(&mut self, _def: &'a ast::TypePrefix) {}
    fn exit_type_prefix(&mut self, _def: &'a ast::TypePrefix) {}

    fn enter_for_index(&mut self, _def: &'a ast::ForIndex) {}
    fn exit_for_index(&mut self, _def: &'a ast::ForIndex) {}

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Simple Terminal Nodes

    fn enter_span(&mut self, _def: &'a ast::Span) {}
    fn exit_span(&mut self, _def: &'a ast::Span) {}

    fn enter_element_flags(&mut self, _def: &'a ast::ElementFlags) {}
    fn exit_element_flags(&mut self, _def: &'a ast::ElementFlags) {}

    fn enter_causality(&mut self, _def: &'a ast::Causality) {}
    fn exit_causality(&mut self, _def: &'a ast::Causality) {}

    fn enter_variability(&mut self, _def: &'a ast::Variability) {}
    fn exit_variability(&mut self, _def: &'a ast::Variability) {}

    fn enter_visibility(&mut self, _def: &'a ast::Visibility) {}
    fn exit_visibility(&mut self, _def: &'a ast::Visibility) {}

    fn enter_connection(&mut self, _def: &'a ast::Connection) {}
    fn exit_connection(&mut self, _def: &'a ast::Connection) {}

    fn enter_unary_op(&mut self, _def: &'a ast::UnaryOp) {}
    fn exit_unary_op(&mut self, _def: &'a ast::UnaryOp) {}

    fn enter_binary_op(&mut self, _def: &'a ast::BinaryOp) {}
    fn exit_binary_op(&mut self, _def: &'a ast::BinaryOp) {}

    fn enter_class_type(&mut self, _def: &'a ast::ClassType) {}
    fn exit_class_type(&mut self, _def: &'a ast::ClassType) {}

    fn enter_name(&mut self, _def: &'a ast::Name) {}
    fn exit_name(&mut self, _def: &'a ast::Name) {}
}

pub trait Visitable<'a> {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V);
}

impl<'a> Visitable<'a> for ast::StoredDefinition {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_stored_definition(self);
        for class in self.classes.iter() {
            class.accept(visitor);
        }
        visitor.exit_stored_definition(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::Span {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_span(self);
        visitor.exit_span(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::ClassDefinition {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_class_definition(self);

        match &self.specifier {
            ast::ClassSpecifier::Long { composition, .. } => {
                for part in composition.iter() {
                    match part {
                        ast::CompositionPart::ElementList { elements, .. } => {
                            for elem in elements.iter() {
                                elem.accept(visitor);
                            }
                        }
                        ast::CompositionPart::AlgorithmSection { statements, .. } => {
                            for stmt in statements.iter() {
                                stmt.accept(visitor);
                            }
                        }
                        ast::CompositionPart::EquationSection { equations, .. } => {
                            for eq in equations.iter() {
                                eq.accept(visitor);
                            }
                        }
                    }
                }
            }
            ast::ClassSpecifier::Extends { .. } => {}
        }
        visitor.exit_class_definition(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::Element {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_element(self);
        match self {
            ast::Element::ComponentClause { clause, .. } => {
                for comp in clause.components.iter() {
                    comp.accept(visitor);
                }
            }
            ast::Element::ClassDefinition { def, .. } => {
                def.accept(visitor);
            }
            ast::Element::ImportClause { .. } => {}
            ast::Element::ExtendsClause { .. } => {}
        }
        visitor.exit_element(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::Equation {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_equation(self);
        match self {
            ast::Equation::Simple { lhs, rhs, .. } => {
                lhs.accept(visitor);
                rhs.accept(visitor);
            }
            ast::Equation::Connect { lhs, rhs, .. } => {
                lhs.accept(visitor);
                rhs.accept(visitor);
            }
            ast::Equation::If {
                if_blocks,
                else_eqs,
                ..
            } => {
                for block in if_blocks.iter() {
                    block.cond.accept(visitor);
                    for block_eq in block.eqs.iter() {
                        block_eq.accept(visitor);
                    }
                }
                for else_eq in else_eqs.iter() {
                    else_eq.accept(visitor);
                }
            }
            ast::Equation::For { eqs, .. } => {
                for eq in eqs.iter() {
                    eq.accept(visitor);
                }
            }
        }
        visitor.exit_equation(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::Statement {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_statement(self);
        match self {
            ast::Statement::If {
                if_blocks,
                else_stmts,
                ..
            } => {
                for block in if_blocks.iter() {
                    block.cond.accept(visitor);
                    for block_stmt in block.stmts.iter() {
                        block_stmt.accept(visitor);
                    }
                }
                for else_stmt in else_stmts.iter() {
                    else_stmt.accept(visitor);
                }
            }
            ast::Statement::For { stmts, .. } => {
                for stmt in stmts.iter() {
                    stmt.accept(visitor);
                }
            }
            ast::Statement::Assignment { comp, rhs, .. } => {
                comp.accept(visitor);
                rhs.accept(visitor);
            }
            ast::Statement::While { cond, stmts, .. } => {
                cond.accept(visitor);
                for stmt in stmts.iter() {
                    stmt.accept(visitor);
                }
            }
            ast::Statement::Break { .. } => {}
            ast::Statement::Return { .. } => {}
        }
        visitor.exit_statement(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::Expression {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_expression(self);
        match self {
            ast::Expression::Ref { comp } => {
                comp.accept(visitor);
            }
            ast::Expression::Unary { rhs, .. } => {
                rhs.accept(visitor);
            }
            ast::Expression::Binary { lhs, rhs, .. } => {
                lhs.accept(visitor);
                rhs.accept(visitor);
            }
            ast::Expression::If {
                if_blocks,
                else_expr,
            } => {
                for block in if_blocks.iter() {
                    block.cond.accept(visitor);
                    block.expr.accept(visitor);
                }
                if let Some(ref expr) = **else_expr {
                    expr.accept(visitor);
                }
            }
            ast::Expression::Der { args } => {
                for arg in args.iter() {
                    arg.accept(visitor);
                }
            }
            ast::Expression::UnsignedInteger(_) => {}
            ast::Expression::UnsignedReal(_) => {}
            ast::Expression::Boolean(_) => {}
            ast::Expression::ArrayArguments { args } => {
                for arg in args.iter() {
                    arg.accept(visitor);
                }
            }
            ast::Expression::FunctionCall { comp, args } => {
                comp.accept(visitor);
                for arg in args.iter() {
                    arg.accept(visitor);
                }
            }
        }
        visitor.exit_expression(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::ComponentReference {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_reference(self);
        visitor.exit_component_reference(self);
        visitor.exit_any();
    }
}

impl<'a> Visitable<'a> for ast::ComponentDeclaration {
    fn accept<V: Visitor<'a> + ?Sized>(&'a self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_declaration(self);
        visitor.exit_component_declaration(self);
        visitor.exit_any();
    }
}
