pub struct Walker;
use crate::s1_parser::ast;
use crate::s1_parser::visitor::Visitor;

impl Walker {
    pub fn walk_stored_definition<'a, V: Visitor<'a>>(
        visitor: &mut V,
        def: &'a ast::StoredDefinition,
    ) {
        visitor.enter_any();
        visitor.enter_stored_definition(def);
        for class in def.classes.iter() {
            Self::walk_class_definition(visitor, class);
        }
        visitor.exit_stored_definition(def);
        visitor.exit_any();
    }

    pub fn walk_class_definition<'a, V: Visitor<'a>>(
        visitor: &mut V,
        class: &'a ast::ClassDefinition,
    ) {
        visitor.enter_any();
        visitor.enter_class_definition(class);

        match &class.specifier {
            ast::ClassSpecifier::Long { composition, .. } => {
                for part in composition.iter() {
                    match part {
                        ast::CompositionPart::ElementList { elements, .. } => {
                            for elem in elements.iter() {
                                Self::walk_element(visitor, elem);
                            }
                        }
                        ast::CompositionPart::AlgorithmSection { statements, .. } => {
                            for stmt in statements.iter() {
                                Self::walk_statement(visitor, stmt);
                            }
                        }
                        ast::CompositionPart::EquationSection { equations, .. } => {
                            for eq in equations.iter() {
                                Self::walk_equation(visitor, eq);
                            }
                        }
                    }
                }
            }
            ast::ClassSpecifier::Extends { .. } => {}
        }
        visitor.exit_class_definition(class);
        visitor.exit_any();
    }

    pub fn walk_element<'a, V: Visitor<'a>>(visitor: &mut V, eq: &'a ast::Element) {
        visitor.enter_any();
        visitor.enter_element(eq);
        visitor.exit_element(eq);
        visitor.exit_any();
    }

    pub fn walk_equation<'a, V: Visitor<'a>>(visitor: &mut V, eq: &'a ast::Equation) {
        visitor.enter_any();
        visitor.enter_equation(eq);
        match eq {
            ast::Equation::Simple { lhs, rhs, .. } => {
                Self::walk_expression(visitor, lhs);
                Self::walk_expression(visitor, rhs);
            }
            ast::Equation::Connect { lhs, rhs, .. } => {
                Self::walk_component_reference(visitor, lhs);
                Self::walk_component_reference(visitor, rhs);
            }
            ast::Equation::If {
                if_blocks,
                else_eqs,
                ..
            } => {
                for block in if_blocks.iter() {
                    Self::walk_expression(visitor, &block.cond);
                    for block_eq in block.eqs.iter() {
                        Self::walk_equation(visitor, block_eq);
                    }
                }
                for else_eq in else_eqs.iter() {
                    Self::walk_equation(visitor, else_eq);
                }
            }
            ast::Equation::For { eqs, .. } => {
                for eq in eqs.iter() {
                    Self::walk_equation(visitor, eq);
                }
            }
        }
        visitor.exit_equation(eq);
        visitor.exit_any();
    }

    pub fn walk_statement<'a, V: Visitor<'a>>(visitor: &mut V, stmt: &'a ast::Statement) {
        visitor.enter_any();
        visitor.enter_statement(stmt);
        match stmt {
            ast::Statement::If {
                if_blocks,
                else_stmts,
                ..
            } => {
                for block in if_blocks.iter() {
                    Self::walk_expression(visitor, &block.cond);
                    for block_stmt in block.stmts.iter() {
                        Self::walk_statement(visitor, block_stmt);
                    }
                }
                for else_stmt in else_stmts.iter() {
                    Self::walk_statement(visitor, else_stmt);
                }
            }
            ast::Statement::For { stmts, .. } => {
                for stmt in stmts.iter() {
                    Self::walk_statement(visitor, stmt);
                }
            }
            ast::Statement::Assignment { comp, rhs, .. } => {
                Self::walk_component_reference(visitor, comp);
                Self::walk_expression(visitor, rhs);
            }
            ast::Statement::While { cond, stmts, .. } => {
                Self::walk_expression(visitor, cond);
                for stmt in stmts.iter() {
                    Self::walk_statement(visitor, stmt);
                }
            }
            ast::Statement::Break { .. } => {}
            ast::Statement::Return { .. } => {}
        }
        visitor.exit_statement(stmt);
        visitor.exit_any();
    }

    pub fn walk_expression<'a, V: Visitor<'a>>(visitor: &mut V, expr: &'a ast::Expression) {
        visitor.enter_any();
        visitor.enter_expression(expr);
        match expr {
            ast::Expression::Ref { comp } => {
                Self::walk_component_reference(visitor, comp);
            }
            ast::Expression::Unary { rhs, .. } => {
                Self::walk_expression(visitor, rhs);
            }
            ast::Expression::Binary { lhs, rhs, .. } => {
                Self::walk_expression(visitor, lhs);
                Self::walk_expression(visitor, rhs);
            }
            ast::Expression::If {
                if_blocks,
                else_expr,
            } => {
                for block in if_blocks.iter() {
                    Self::walk_expression(visitor, &block.cond);
                    Self::walk_expression(visitor, &block.expr);
                }
                if let Some(ref expr) = **else_expr {
                    Self::walk_expression(visitor, expr);
                }
            }
            ast::Expression::Der { args } => {
                for arg in args.iter() {
                    Self::walk_expression(visitor, arg);
                }
            }
            ast::Expression::UnsignedInteger(_) => {}
            ast::Expression::UnsignedReal(_) => {}
            ast::Expression::Boolean(_) => {}
            ast::Expression::ArrayArguments { args } => {
                for arg in args.iter() {
                    Self::walk_expression(visitor, arg);
                }
            }
            ast::Expression::FunctionCall { comp, args } => {
                Self::walk_component_reference(visitor, comp);
                for arg in args.iter() {
                    Self::walk_expression(visitor, arg);
                }
            }
        }
        visitor.exit_expression(expr);
        visitor.exit_any();
    }

    pub fn walk_component_reference<'a, V: Visitor<'a>>(
        visitor: &mut V,
        comp: &'a ast::ComponentReference,
    ) {
        visitor.enter_any();
        visitor.enter_component_reference(comp);
        visitor.exit_component_reference(comp);
        visitor.exit_any();
    }
}
