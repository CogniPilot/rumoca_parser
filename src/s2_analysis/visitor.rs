use crate::s1_parser::ast;
use paste::paste;

macro_rules! define_visitor_methods {
    ($($name:ident),*) => {
        paste! {
            $(
                fn [<enter_ $name:snake>](&mut self, _def: &mut ast::$name) {}
                fn [<exit_ $name:snake>](&mut self, _def: &mut ast::$name) {}
            )*
        }
    };
}

pub trait Visitor {
    fn enter_any(&mut self) {}
    fn exit_any(&mut self) {}

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
        ArraySubscripts,
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
