use super::super::s1_parser::ast::*;
use paste::paste;

macro_rules! define_node_enums {
    ($($name:ident),*) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
        pub enum Node {
            #[default]
            Empty,
            $(
                $name($name),
            )*
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
        pub enum NodeRef<'a> {
            #[default]
            Empty,
            $(
                $name(&'a $name),
            )*
        }

        #[derive(Debug, PartialEq, Eq, Hash, Default)]
        pub enum NodeMutRef<'a> {
            #[default]
            Empty,
            $(
                $name(& 'a mut $name),
            )*
        }
    };
}

macro_rules! from_node {
    ($($variant:ident),*) => {
        paste! {
            $(
                impl From<Node> for $variant {
                    fn from(value: Node) -> Self {
                        if let Node::$variant(inner) = value {
                            inner
                        } else {
                            panic!("Expected Node::{}", stringify!($variant));
                        }
                    }
                }

                impl <'a> From<&'a Node> for &'a $variant {
                    fn from(value: &'a Node) -> Self {
                        if let Node::$variant(inner) = value {
                            inner
                        } else {
                            panic!("Expected Node::{}", stringify!($variant));
                        }
                    }
                }

                impl <'a> From<&'a mut Node> for &'a mut $variant {
                    fn from(value: &'a mut Node) -> Self {
                        if let Node::$variant(inner) = value {
                            inner
                        } else {
                            panic!("Expected Node::{}", stringify!($variant));
                        }
                    }
                }

                impl From<$variant> for Node {
                    fn from(value: $variant) -> Self {
                        Node::$variant(value)
                    }
                }

                impl <'a> From<&'a $variant> for NodeRef<'a> {
                    fn from(value: &'a $variant) -> Self {
                        NodeRef::$variant(value)
                    }
                }

                impl <'a> From<NodeRef<'a>>  for &'a $variant {
                    fn from(value: NodeRef<'a>) -> Self {
                        if let NodeRef::$variant(inner) = value {
                            inner
                        } else {
                            panic!("Expected NodeRef::{}", stringify!($variant));
                        }
                    }
                }

                impl <'a> From<&'a mut $variant> for NodeMutRef<'a> {
                    fn from(value: &'a mut $variant) -> Self {
                        NodeMutRef::$variant(value)
                    }
                }

                impl <'a> From<NodeMutRef<'a>>  for &'a mut $variant {
                    fn from(value: NodeMutRef<'a>) -> Self {
                        if let NodeMutRef::$variant(inner) = value {
                            inner
                        } else {
                            panic!("Expected NodeMutRef::{}", stringify!($variant));
                        }
                    }
                }
            )*
        }
    };
}

from_node!(
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

// Usage:
define_node_enums!(
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
