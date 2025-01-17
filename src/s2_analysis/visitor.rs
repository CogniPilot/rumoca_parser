use crate::s1_parser::ast;
use paste::paste;

macro_rules! define_visitor_methods {
    ($($name:ident),*) => {
        paste! {
            $(
                #[allow(unused_variables)]
                fn [<enter_ $name:snake>](&mut self, node: &mut ast::$name) {}
                #[allow(unused_variables)]
                fn [<exit_ $name:snake>](&mut self, node: &mut ast::$name) {}
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
