use serde::{Deserialize, Serialize};

derive_alias! {
    #[derive(CommonTraits!)] = #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash)];
}

#[derive(CommonTraits!, Default)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}

#[derive(CommonTraits!, Default)]
pub struct StoredDefinition {
    pub span: Span,
    pub classes: Vec<ClassDefinition>,
    pub within: Option<Name>,
    pub model_md5: String,
    pub rumoca_parser_version: String,
    pub rumoca_parser_git: String,
}

#[derive(CommonTraits!, Default)]
pub struct ClassDefinition {
    pub is_encapsulated: bool,
    pub is_final: bool,
    pub prefixes: ClassPrefixes,
    pub specifier: ClassSpecifier,
}

#[derive(CommonTraits!)]
pub struct ComponentDeclaration {
    pub declaration: Declaration,
    pub condition_attribute: Option<Expression>,
    pub description: Description,
}

#[derive(CommonTraits!)]
pub struct ComponentDeclaration1 {
    pub declaration: Declaration,
    pub description: Description,
}

#[derive(CommonTraits!, Default)]
pub struct ClassPrefixes {
    pub is_partial: bool,
    pub class_type: ClassType,
}

#[derive(CommonTraits!)]
pub enum ClassSpecifier {
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

impl Default for ClassSpecifier {
    fn default() -> Self {
        ClassSpecifier::Long {
            name: "".to_string(),
            description: Vec::new(),
            composition: Vec::new(),
            name_end: "".to_string(),
        }
    }
}

#[derive(CommonTraits!)]
pub enum CompositionPart {
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

#[derive(CommonTraits!)]
pub struct ComponentClause {
    pub type_prefix: TypePrefix,
    pub type_specifier: TypeSpecifier,
    pub array_subscripts: Option<Vec<Subscript>>,
    pub components: Vec<ComponentDeclaration>,
}

#[derive(CommonTraits!)]
pub struct ElementFlags {
    pub replaceable: bool,
    pub redeclare: bool,
    pub final_: bool,
    pub inner: bool,
    pub outer: bool,
}

#[derive(CommonTraits!)]
pub enum Element {
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

#[derive(CommonTraits!)]
pub enum Causality {
    None,
    Input,
    Output,
}

#[derive(CommonTraits!)]
pub enum Variability {
    Continuous,
    Discrete,
    Parameter,
    Constant,
}

#[derive(CommonTraits!)]
pub enum Visibility {
    Public,
    Protected,
}

#[derive(CommonTraits!)]
pub enum Connection {
    None,
    Flow,
    Stream,
}

#[derive(CommonTraits!)]
pub enum Statement {
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

#[derive(CommonTraits!)]
pub struct RefPart {
    pub name: String,
    pub array_subscripts: Vec<Subscript>,
}

#[derive(CommonTraits!)]
pub struct ComponentReference {
    pub local: bool,
    pub parts: Vec<RefPart>,
}

#[derive(CommonTraits!)]
pub enum Equation {
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

#[derive(CommonTraits!)]
pub struct ForIndex {
    pub ident: String,
    pub in_expr: Option<Expression>,
}

#[derive(CommonTraits!)]
pub struct IfEquationBlock {
    pub cond: Expression,
    pub eqs: Vec<Equation>,
}

#[derive(CommonTraits!)]
pub struct IfStatementBlock {
    pub cond: Expression,
    pub stmts: Vec<Statement>,
}

#[derive(CommonTraits!)]
pub struct IfExpressionBlock {
    pub cond: Expression,
    pub expr: Expression,
}

#[derive(CommonTraits!)]
pub enum UnaryOp {
    Paren,
    Not,
    Negative,
}

#[derive(CommonTraits!)]
pub enum BinaryOp {
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

#[derive(CommonTraits!)]
pub enum Expression {
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
pub struct Description {
    pub strings: Vec<String>,
    pub annotation: Vec<Argument>,
}

#[derive(CommonTraits!)]
pub enum ModExpr {
    Break,
    Expression { expr: Expression },
}

#[derive(CommonTraits!)]
pub enum Modification {
    Expression {
        expr: ModExpr,
    },
    Class {
        args: Vec<Argument>,
        expr: Option<ModExpr>,
    },
}

#[derive(CommonTraits!)]
pub struct Declaration {
    pub name: String,
    pub array_subscripts: Option<Vec<Subscript>>,
    pub modification: Option<Modification>,
}

#[derive(CommonTraits!)]
pub enum Argument {
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

#[derive(CommonTraits!)]
pub struct TypePrefix {
    pub connection: Connection,
    pub variability: Variability,
    pub causality: Causality,
}

#[derive(CommonTraits!)]
pub struct ComponentClause1 {
    pub type_prefix: TypePrefix,
    pub type_specifier: TypeSpecifier,
    pub component_declaration1: ComponentDeclaration1,
}

#[derive(CommonTraits!)]
pub struct Name {
    pub ident: Vec<String>,
}

#[derive(CommonTraits!)]
pub struct TypeSpecifier {
    pub leading_period: bool,
    pub name: Name,
}

#[derive(CommonTraits!)]
pub enum Subscript {
    Colon,
    Expression(Expression),
}

#[derive(CommonTraits!, Default)]
pub enum ClassType {
    #[default]
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
