use crate::s1_parser::ast;

pub trait Visitor<'a> {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

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

    fn enter_component_declaration(&mut self, _comp: &'a ast::ComponentDeclaration) {}
    fn exit_component_declaration(&mut self, _comp: &'a ast::ComponentDeclaration) {}

    fn walk<T: Visitable<'a> + ?Sized>(&'a mut self, node: &'a T) {
        node.accept(self);
    }
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
