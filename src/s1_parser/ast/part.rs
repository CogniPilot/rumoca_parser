//! This module contains AST parts.
//!
//! Parts are not considered nodes, but they are contained in the
//! final AST, unlike fragments.
use serde::{Deserialize, Serialize};

derive_alias! {
    #[derive(CommonTraits!)] = #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)];
}

#[derive(CommonTraits!, Default)]
pub struct ParserContext {
    id_count: usize,
}

impl ParserContext {
    pub fn new_id(&mut self) -> usize {
        let id = self.id_count;
        self.id_count += 1;
        id
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common Node Data

#[derive(CommonTraits!, Default)]
pub struct NodeData {
    pub id: usize,
    pub span: (usize, usize),
}

impl NodeData {
    pub fn new(id: usize, left: usize, right: usize) -> Self {
        NodeData {
            id,
            span: (left, right),
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Level Parts

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Class Level Parts

#[derive(CommonTraits!, Default)]
pub struct ClassFlags {
    pub partial: bool,
    pub encapsulated: bool,
    pub replaceable: bool,
    pub redeclare: bool,
    pub is_final: bool,
    pub inner: bool,
    pub outer: bool,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equations

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statements

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expressions

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Modification

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Simple Terminals

#[derive(CommonTraits!, Default)]
pub struct ElementFlags {
    pub replaceable: bool,
    pub redeclare: bool,
    pub is_final: bool,
    pub inner: bool,
    pub outer: bool,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum Causality {
    #[default]
    Empty,
    Input,
    Output,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum Variability {
    #[default]
    Empty,
    Constant,
    Continuous,
    Discrete,
    Parameter,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum Visibility {
    #[default]
    Empty,
    Public,
    Protected,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum Connection {
    #[default]
    Empty,
    Flow,
    Stream,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum UnaryOp {
    #[default]
    Empty,
    ElemNegative,
    ElemPositive,
    Negative,
    Not,
    Paren,
    Positive,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum BinaryOp {
    #[default]
    Empty,
    Add,
    And,
    Div,
    ElemAdd,
    ElemDiv,
    ElemExp,
    ElemMul,
    ElemSub,
    Equal,
    Exp,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Mul,
    Not,
    NotEqual,
    Or,
    Paren,
    Range,
    Sub,
}

#[derive(CommonTraits!, Default, Debug)]
pub enum ClassType {
    #[default]
    Empty,
    Block,
    Class,
    Connector,
    ExpandableConnector,
    Function,
    ImpureFunction,
    Model,
    Operator,
    OperatorFunction,
    OperatorRecord,
    Package,
    PureFunction,
    Record,
    Type,
}

#[derive(CommonTraits!, Default)]
pub struct DescriptionString {
    pub parts: Vec<String>,
}

#[derive(CommonTraits!, Default)]
pub struct Name {
    pub parts: Vec<String>,
}
