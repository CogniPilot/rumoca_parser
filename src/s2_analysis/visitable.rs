use super::visitor::Visitor;
use crate::s1_parser::ast;
use paste::paste;

pub trait Visitable {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Level Nodes

impl Visitable for ast::StoredDefinition {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_stored_definition(self);
        for class in self.classes.iter_mut() {
            class.accept(visitor);
        }
        visitor.exit_stored_definition(self);
        visitor.exit_any();
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Class Level Node

impl Visitable for ast::ClassDefinition {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_class_definition(self);
        self.specifier.accept(visitor);
        visitor.exit_class_definition(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ClassSpecifier {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_class_specifier(self);
        match self {
            ast::ClassSpecifier::Long { composition, .. } => {
                for part in composition.iter_mut() {
                    part.accept(visitor);
                }
            }
            ast::ClassSpecifier::Extends {
                modification,
                composition,
                ..
            } => {
                if let Some(modifs) = modification {
                    for modif in modifs.iter_mut() {
                        modif.accept(visitor);
                    }
                }
                for part in composition.iter_mut() {
                    part.accept(visitor);
                }
            }
            ast::ClassSpecifier::Empty => {}
        }
        visitor.exit_class_specifier(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::CompositionPart {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_composition_part(self);
        match self {
            ast::CompositionPart::ElementList {
                visibility,
                elements,
            } => {
                visibility.accept(visitor);
                for elem in elements.iter_mut() {
                    elem.accept(visitor);
                }
            }
            ast::CompositionPart::AlgorithmSection { statements, .. } => {
                for stmt in statements.iter_mut() {
                    stmt.accept(visitor);
                }
            }
            ast::CompositionPart::EquationSection {
                span, equations, ..
            } => {
                span.accept(visitor);
                for eq in equations.iter_mut() {
                    eq.accept(visitor);
                }
            }
            ast::CompositionPart::Empty => {}
        }
        visitor.exit_composition_part(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::Element {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_element(self);
        match self {
            ast::Element::ComponentClause { flags, clause } => {
                flags.accept(visitor);
                clause.accept(visitor);
            }
            ast::Element::ClassDefinition { flags, def } => {
                flags.accept(visitor);
                def.accept(visitor);
            }
            ast::Element::ImportClause {
                name, description, ..
            } => {
                name.accept(visitor);
                description.accept(visitor);
            }
            ast::Element::ExtendsClause { type_specifier } => {
                type_specifier.accept(visitor);
            }
            ast::Element::Empty => {}
        }
        visitor.exit_element(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ComponentDeclaration {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_declaration(self);
        self.declaration.accept(visitor);
        if let Some(expr) = self.condition_attribute.as_mut() {
            expr.accept(visitor);
        }
        self.description.accept(visitor);
        visitor.exit_component_declaration(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ComponentDeclaration1 {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_declaration1(self);
        self.declaration.accept(visitor);
        self.description.accept(visitor);
        visitor.exit_component_declaration1(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ClassPrefixes {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_class_prefixes(self);
        self.class_type.accept(visitor);
        visitor.exit_class_prefixes(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ComponentClause {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_clause(self);
        self.type_prefix.accept(visitor);
        self.type_specifier.accept(visitor);
        if let Some(subs) = self.array_subscripts.as_mut() {
            subs.accept(visitor);
        }
        for comp in self.components.iter_mut() {
            comp.accept(visitor);
        }
        visitor.exit_component_clause(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ComponentClause1 {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_clause1(self);
        self.type_prefix.accept(visitor);
        self.type_specifier.accept(visitor);
        self.component_declaration1.accept(visitor);
        visitor.exit_component_clause1(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::Declaration {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_declaration(self);
        if let Some(subs) = self.array_subscripts.as_mut() {
            subs.accept(visitor);
        }
        if let Some(modif) = self.modification.as_mut() {
            modif.accept(visitor);
        }
        visitor.exit_declaration(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::TypeSpecifier {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_type_specifier(self);
        self.name.accept(visitor);
        visitor.exit_type_specifier(self);
        visitor.exit_any();
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equation Nodes

impl Visitable for ast::Equation {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_equation(self);
        match self {
            ast::Equation::Simple {
                lhs,
                rhs,
                description,
            } => {
                lhs.accept(visitor);
                rhs.accept(visitor);
                description.accept(visitor);
            }
            ast::Equation::Connect {
                lhs,
                rhs,
                description,
            } => {
                lhs.accept(visitor);
                rhs.accept(visitor);
                description.accept(visitor);
            }
            ast::Equation::If {
                if_blocks,
                else_eqs,
                description,
            } => {
                for block in if_blocks.iter_mut() {
                    block.cond.accept(visitor);
                    for block_eq in block.eqs.iter_mut() {
                        block_eq.accept(visitor);
                    }
                }
                for else_eq in else_eqs.iter_mut() {
                    else_eq.accept(visitor);
                }
                description.accept(visitor);
            }
            ast::Equation::For {
                indices,
                eqs,
                description,
            } => {
                for index in indices.iter_mut() {
                    index.accept(visitor);
                }
                for eq in eqs.iter_mut() {
                    eq.accept(visitor);
                }
                description.accept(visitor);
            }
            ast::Equation::Empty => {}
        }
        visitor.exit_equation(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::IfEquationBlock {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        self.cond.accept(visitor);
        for eq in self.eqs.iter_mut() {
            eq.accept(visitor);
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statement Nodes

impl Visitable for ast::Statement {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_statement(self);
        match self {
            ast::Statement::If {
                if_blocks,
                else_stmts,
                ..
            } => {
                for block in if_blocks.iter_mut() {
                    block.cond.accept(visitor);
                    for block_stmt in block.stmts.iter_mut() {
                        block_stmt.accept(visitor);
                    }
                }
                for else_stmt in else_stmts.iter_mut() {
                    else_stmt.accept(visitor);
                }
            }
            ast::Statement::For { stmts, .. } => {
                for stmt in stmts.iter_mut() {
                    stmt.accept(visitor);
                }
            }
            ast::Statement::Assignment { comp, rhs, .. } => {
                comp.accept(visitor);
                rhs.accept(visitor);
            }
            ast::Statement::While { cond, stmts, .. } => {
                cond.accept(visitor);
                for stmt in stmts.iter_mut() {
                    stmt.accept(visitor);
                }
            }
            ast::Statement::Break { .. } => {}
            ast::Statement::Return { .. } => {}
            ast::Statement::Empty => {}
        }
        visitor.exit_statement(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::IfStatementBlock {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        self.cond.accept(visitor);
        for stmt in self.stmts.iter_mut() {
            stmt.accept(visitor);
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expression Nodes

impl Visitable for ast::Expression {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
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
                for block in if_blocks.iter_mut() {
                    block.cond.accept(visitor);
                    block.expr.accept(visitor);
                }
                if let Some(ref mut expr) = **else_expr {
                    expr.accept(visitor);
                }
            }
            ast::Expression::Der { args } => {
                for arg in args.iter_mut() {
                    arg.accept(visitor);
                }
            }
            ast::Expression::UnsignedInteger(_) => {}
            ast::Expression::UnsignedReal(_) => {}
            ast::Expression::Boolean(_) => {}
            ast::Expression::ArrayArguments { args } => {
                for arg in args.iter_mut() {
                    arg.accept(visitor);
                }
            }
            ast::Expression::FunctionCall { comp, args } => {
                comp.accept(visitor);
                for arg in args.iter_mut() {
                    arg.accept(visitor);
                }
            }
            ast::Expression::Empty => {}
        }
        visitor.exit_expression(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::IfExpressionBlock {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        self.cond.accept(visitor);
        self.expr.accept(visitor);
    }
}

impl Visitable for ast::ComponentReference {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_component_reference(self);
        for part in self.parts.iter_mut() {
            part.accept(visitor);
        }
        visitor.exit_component_reference(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::RefPart {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_ref_part(self);
        if let Some(subs) = self.array_subscripts.as_mut() {
            subs.accept(visitor);
        }
        visitor.exit_ref_part(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ArraySubscripts {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_array_subscripts(self);
        for sub in self.sub.iter_mut() {
            sub.accept(visitor);
        }
        visitor.exit_array_subscripts(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::Subscript {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_subscript(self);
        match self {
            ast::Subscript::Expression(expr) => {
                expr.accept(visitor);
            }
            ast::Subscript::Colon => {}
            ast::Subscript::Empty => {}
        }
        visitor.exit_subscript(self);
        visitor.exit_any();
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Modification Nodes

impl Visitable for ast::Argument {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_argument(self);
        match self {
            ast::Argument::Modification {
                name, modification, ..
            } => {
                name.accept(visitor);
                if let Some(modif) = modification.as_mut() {
                    modif.accept(visitor);
                }
            }
            ast::Argument::Replaceable => {}
            ast::Argument::Redeclaration => {}
            ast::Argument::Empty => {}
        }
        visitor.exit_argument(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::Modification {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_modification(self);
        match self {
            ast::Modification::Expression { expr } => {
                expr.accept(visitor);
            }
            ast::Modification::Class { args, expr } => {
                for arg in args.iter_mut() {
                    arg.accept(visitor);
                }
                if let Some(modif) = expr.as_mut() {
                    modif.accept(visitor);
                }
            }
            ast::Modification::Empty => {}
        }
        visitor.exit_modification(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ModExpr {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_mod_expr(self);
        match self {
            ast::ModExpr::Break => {}
            ast::ModExpr::Expression { expr } => {
                expr.accept(visitor);
            }
            ast::ModExpr::Empty => {}
        }
        visitor.exit_mod_expr(self);
        visitor.exit_any();
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common Nodes

impl Visitable for ast::Description {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_description(self);
        for arg in self.annotation.iter_mut() {
            arg.accept(visitor);
        }
        visitor.exit_description(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::TypePrefix {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.enter_any();
        visitor.enter_type_prefix(self);
        self.connection.accept(visitor);
        self.variability.accept(visitor);
        self.causality.accept(visitor);
        visitor.exit_type_prefix(self);
        visitor.exit_any();
    }
}

impl Visitable for ast::ForIndex {
    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
        if let Some(expr) = self.in_expr.as_mut() {
            expr.accept(visitor);
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Simple Terminal Nodes

macro_rules! accept_terminal_node {
    ($($name:ident),*) => {
        paste! {
            $(
                impl Visitable for ast::$name {
                    fn accept<V: Visitor + ?Sized>(&mut self, visitor: &mut V) {
                        visitor.enter_any();
                        visitor.[<enter_ $name:snake>](self);
                        visitor.[<exit_ $name:snake>](self);
                        visitor.exit_any();
                    }
                }
            )*
        }
    };
}

accept_terminal_node!(
    Span,
    ElementFlags,
    Causality,
    Variability,
    Visibility,
    Connection,
    UnaryOp,
    BinaryOp,
    ClassType,
    Name
);
