use super::super::s1_parser::ast;
use super::ast_node::NodeRef;

use paste::paste;

macro_rules! define_visitor_methods {
    ($($name:ident),*) => {
        paste! {
            $(
                fn [<enter_ $name:snake>](&mut self, node: &'a ast::$name) {}
                fn [<exit_ $name:snake>](&mut self, node: &'a ast::$name) {}
            )*
        }
    };
}

#[allow(unused_variables)]
pub trait Visitor<'a> {
    fn enter_any(&mut self, node: NodeRef<'a>) {}
    fn exit_any(&mut self, node: NodeRef<'a>) {}

    define_visitor_methods!(
        StoredDefinition,
        ClassDefinition,
        ClassSpecifier,
        CompositionPart,
        Element,
        ComponentDeclaration,
        ComponentDeclaration1,
        ClassPrefixes,
        ComponentClause,
        ComponentClause1,
        Declaration,
        TypeSpecifier,
        Equation,
        IfEquationBlock,
        Statement,
        IfStatementBlock,
        Expression,
        IfExpressionBlock,
        ComponentReference,
        RefPart,
        Subscript,
        Argument,
        Modification,
        ModExpr,
        Description,
        TypePrefix,
        ForIndex,
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
}
