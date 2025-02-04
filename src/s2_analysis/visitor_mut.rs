use super::ast_node::NodeRef;
use crate::s1_parser::ast;
use paste::paste;

macro_rules! define_visitor_mut_methods {
    ($($name:ident),*) => {
        paste! {
            $(
                fn [<enter_ $name:snake _mut>](&mut self, node: & mut ast::$name) {}
                fn [<exit_ $name:snake _mut>](&mut self, node: & mut ast::$name) {}
            )*
        }
    };
}

#[allow(unused_variables)]
pub trait VisitorMut {
    fn enter_any(&mut self, node: NodeRef) {}
    fn exit_any(&mut self, node: NodeRef) {}

    define_visitor_mut_methods!(
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
        ArraySubscripts,
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
