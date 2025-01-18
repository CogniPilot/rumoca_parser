use serde::{Deserialize, Serialize};

derive_alias! {
    #[derive(CommonTraits!)] = #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)];
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Level Nodes

#[derive(CommonTraits!, Default)]
pub struct StoredDefinition {
    pub span: Span,
    pub classes: Vec<ClassDefinition>,
    pub within: Option<Name>,
    pub model_md5: String,
    pub rumoca_parser_version: String,
    pub rumoca_parser_git: String,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Class Level Nodes

#[derive(CommonTraits!, Default)]
pub struct ClassDefinition {
    pub is_encapsulated: bool,
    pub is_final: bool,
    pub prefixes: ClassPrefixes,
    pub specifier: ClassSpecifier,
}

#[derive(CommonTraits!, Default)]
pub enum ClassSpecifier {
    #[default]
    Empty,
    Long {
        name: String,
        description: Vec<String>,
        composition: Vec<CompositionPart>,
        name_end: String,
    },
    Extends {
        name: String,
        modification: Option<Vec<Argument>>,
        description: Vec<String>,
        composition: Vec<CompositionPart>,
        name_end: String,
    },
}

#[derive(CommonTraits!, Default)]
pub enum CompositionPart {
    #[default]
    Empty,
    ElementList {
        visibility: Visibility,
        elements: Vec<Element>,
    },
    EquationSection {
        span: Span,
        initial: bool,
        equations: Vec<Equation>,
    },
    AlgorithmSection {
        initial: bool,
        statements: Vec<Statement>,
    },
}

#[derive(CommonTraits!, Default)]
pub enum Element {
    #[default]
    Empty,
    ImportClause {
        alias: String,
        name: Name,
        description: Description,
    },
    ComponentClause {
        flags: ElementFlags,
        clause: ComponentClause,
    },
    ClassDefinition {
        flags: ElementFlags,
        def: ClassDefinition,
    },
    ExtendsClause {
        type_specifier: TypeSpecifier,
    },
}

#[derive(CommonTraits!, Default)]
pub struct ComponentDeclaration {
    pub declaration: Declaration,
    pub condition_attribute: Option<Expression>,
    pub description: Description,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentDeclaration1 {
    pub declaration: Declaration,
    pub description: Description,
}

#[derive(CommonTraits!, Default)]
pub struct ClassPrefixes {
    pub is_partial: bool,
    pub class_type: ClassType,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentClause {
    pub type_prefix: TypePrefix,
    pub type_specifier: TypeSpecifier,
    pub array_subscripts: Option<ArraySubscripts>,
    pub components: Vec<ComponentDeclaration>,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentClause1 {
    pub type_prefix: TypePrefix,
    pub type_specifier: TypeSpecifier,
    pub component_declaration1: ComponentDeclaration1,
}

#[derive(CommonTraits!, Default)]
pub struct Declaration {
    pub name: String,
    pub array_subscripts: Option<ArraySubscripts>,
    pub modification: Option<Modification>,
}

#[derive(CommonTraits!, Default)]
pub struct TypeSpecifier {
    pub leading_period: bool,
    pub name: Name,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equations

#[derive(CommonTraits!, Default)]
pub enum Equation {
    #[default]
    Empty,
    Simple {
        lhs: Expression,
        rhs: Expression,
        description: Description,
    },
    If {
        if_blocks: Vec<IfEquationBlock>,
        else_eqs: Vec<Equation>,
        description: Description,
    },
    For {
        indices: Vec<ForIndex>,
        eqs: Vec<Equation>,
        description: Description,
    },
    Connect {
        lhs: ComponentReference,
        rhs: ComponentReference,
        description: Description,
    },
}

#[derive(CommonTraits!, Default)]
pub struct IfEquationBlock {
    pub cond: Expression,
    pub eqs: Vec<Equation>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statements

#[derive(CommonTraits!, Default)]
pub enum Statement {
    #[default]
    Empty,
    Assignment {
        comp: ComponentReference,
        rhs: Expression,
        description: Description,
    },
    If {
        if_blocks: Vec<IfStatementBlock>,
        else_stmts: Vec<Statement>,
        description: Description,
    },
    For {
        indices: Vec<ForIndex>,
        stmts: Vec<Statement>,
        description: Description,
    },
    While {
        cond: Expression,
        stmts: Vec<Statement>,
        description: Description,
    },
    Break {
        description: Description,
    },
    Return {
        description: Description,
    },
}

#[derive(CommonTraits!, Default)]
pub struct IfStatementBlock {
    pub cond: Expression,
    pub stmts: Vec<Statement>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expressions

#[derive(CommonTraits!, Default)]
pub enum Expression {
    #[default]
    Empty,
    UnsignedInteger(String),
    UnsignedReal(String),
    Boolean(bool),
    //String(String),
    Ref {
        comp: ComponentReference,
    },
    Unary {
        op: UnaryOp,
        rhs: Box<Expression>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    If {
        if_blocks: Vec<IfExpressionBlock>,
        else_expr: Box<Option<Expression>>,
    },
    ArrayArguments {
        args: Vec<Expression>,
    },
    FunctionCall {
        comp: ComponentReference,
        args: Vec<Expression>,
    },
    Der {
        args: Vec<Expression>,
    },
}

#[derive(CommonTraits!, Default)]
pub struct IfExpressionBlock {
    pub cond: Expression,
    pub expr: Expression,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentReference {
    pub local: bool,
    pub parts: Vec<RefPart>,
}

#[derive(CommonTraits!, Default)]
pub struct RefPart {
    pub name: String,
    pub array_subscripts: Option<ArraySubscripts>,
}

#[derive(CommonTraits!, Default)]
pub struct ArraySubscripts {
    pub sub: Vec<Subscript>,
}

#[derive(CommonTraits!, Default)]
pub enum Subscript {
    #[default]
    Empty,
    Colon,
    Expression(Expression),
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Modification

#[derive(CommonTraits!, Default)]
pub enum Argument {
    #[default]
    Empty,
    Modification {
        name: Name,
        each: bool,
        is_final: bool,
        modification: Option<Modification>,
        description: Description,
    },
    Replaceable,
    Redeclaration,
}

#[derive(CommonTraits!, Default)]
pub enum Modification {
    #[default]
    Empty,
    Expression {
        expr: ModExpr,
    },
    Class {
        args: Vec<Argument>,
        expr: Option<ModExpr>,
    },
}

#[derive(CommonTraits!, Default)]
pub enum ModExpr {
    #[default]
    Empty,
    Break,
    Expression {
        expr: Expression,
    },
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common

#[derive(CommonTraits!, Default)]
pub struct Description {
    pub strings: Vec<String>,
    pub annotation: Vec<Argument>,
}

#[derive(CommonTraits!, Default)]
pub struct TypePrefix {
    pub connection: Connection,
    pub variability: Variability,
    pub causality: Causality,
}

#[derive(CommonTraits!, Default)]
pub struct ForIndex {
    pub ident: String,
    pub in_expr: Option<Expression>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Simple Terminal Nodes

#[derive(CommonTraits!, Default)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}

#[derive(CommonTraits!, Default)]
pub struct ElementFlags {
    pub replaceable: bool,
    pub redeclare: bool,
    pub final_: bool,
    pub inner: bool,
    pub outer: bool,
}

#[derive(CommonTraits!, Default)]
pub enum Causality {
    #[default]
    Empty,
    Input,
    Output,
}

#[derive(CommonTraits!, Default)]
pub enum Variability {
    #[default]
    Empty,
    Continuous,
    Discrete,
    Parameter,
    Constant,
}

#[derive(CommonTraits!, Default)]
pub enum Visibility {
    #[default]
    Empty,
    Public,
    Protected,
}

#[derive(CommonTraits!, Default)]
pub enum Connection {
    #[default]
    Empty,
    Flow,
    Stream,
}

#[derive(CommonTraits!, Default)]
pub enum UnaryOp {
    #[default]
    Empty,
    Paren,
    Not,
    Negative,
    Positive,
    ElemNegative,
    ElemPositive,
}

#[derive(CommonTraits!, Default)]
pub enum BinaryOp {
    #[default]
    Empty,
    Paren,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    ElemAdd,
    ElemSub,
    ElemMul,
    ElemDiv,
    Exp,
    ElemExp,
    Or,
    And,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    Range,
}

#[derive(CommonTraits!, Default)]
pub enum ClassType {
    #[default]
    Empty,
    Class,
    Model,
    Record,
    OperatorRecord,
    Block,
    ExpandableConnector,
    Connector,
    Type,
    Package,
    PureFunction,
    ImpureFunction,
    OperatorFunction,
    Function,
    Operator,
}

#[derive(CommonTraits!, Default)]
pub struct Name {
    pub ident: Vec<String>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Unit Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast() {
        let mut def = StoredDefinition::default();

        // class ball
        let class_ball = ClassDefinition {
            ..Default::default()
        };
        def.classes.push(class_ball);
    }
}
