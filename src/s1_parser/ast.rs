use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

derive_alias! {
    #[derive(CommonTraits!)] = #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)];
}

macro_rules! impl_debug_for_enum {
    ($enum_name:ident { $($variant:ident),* $(,)? }) => {
        impl fmt::Debug for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$variant(v) => {
                            if f.alternate() {
                                write!(f, "{:#?}", v)
                            } else {
                                write!(f, "{:?}", v)
                            }
                        }
                    )*
                    _ => Ok(()), // Default case for ignored variants
                }
            }
        }
    };
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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

impl fmt::Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeData {{id: {:?}, span: {:?}}}", self.id, self.span)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Level Nodes
#[derive(CommonTraits!, Default)]
pub struct StoredDefinition {
    pub node_data: NodeData,
    pub within: Option<Name>,
    pub model_md5: String,
    pub rumoca_parser_version: String,
    pub rumoca_parser_git: String,
    pub classes: IndexMap<String, ClassDefinition>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Class Level Nodes

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClassFlags {
    pub partial: bool,
    pub encapsulated: bool,
    pub replaceable: bool,
    pub redeclare: bool,
    pub is_final: bool,
    pub inner: bool,
    pub outer: bool,
}

impl fmt::Debug for ClassFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = [
            ("partial", self.partial),
            ("encapsulated", self.encapsulated),
            ("replaceable", self.replaceable),
            ("redeclare", self.redeclare),
            ("is_final", self.is_final),
            ("inner", self.inner),
            ("outer", self.outer),
        ]
        .iter()
        .filter_map(|&(flag, is_set)| if is_set { Some(flag) } else { None })
        .collect::<Vec<_>>(); // Collect names of true flags into a vector
        write!(f, "[{}]", flags.join(", "))
    }
}

#[derive(CommonTraits!, Default)]
pub struct ClassDefinition {
    pub node_data: NodeData,
    pub name: String,
    pub class_type: ClassType,
    pub extends: Vec<TypeSpecifier>,
    pub imports: Vec<ImportClause>,
    pub flags: ClassFlags,
    pub modification: Vec<Argument>,
    pub description: DescriptionString,
    pub components: IndexMap<String, ComponentDeclaration>,
    pub classes: IndexMap<String, ClassDefinition>,
    pub equations: Vec<Equation>,
    pub algorithms: Vec<Vec<Statement>>,
    pub initial_equations: Vec<Equation>,
    pub initial_algorithms: Vec<Vec<Statement>>,
}

#[derive(CommonTraits!, Default)]
pub enum ClassSpecifier {
    #[default]
    Empty,
    Extends(ClassSpecifierExtends),
    Long(ClassSpecifierLong),
}

#[derive(CommonTraits!, Default)]
pub struct ClassSpecifierLong {
    pub node_data: NodeData,
    pub name: String,
    pub description: DescriptionString,
    pub composition: Vec<CompositionPart>,
    pub name_end: String,
}

#[derive(CommonTraits!, Default)]
pub struct ClassSpecifierExtends {
    pub node_data: NodeData,
    pub name: String,
    pub modification: Vec<Argument>,
    pub description: DescriptionString,
    pub composition: Vec<CompositionPart>,
    pub name_end: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CompositionPart {
    #[default]
    Empty,
    AlgorithmSection(AlgorithmSection),
    ElementList(ElementList),
    EquationSection(EquationSection),
}

impl_debug_for_enum!(CompositionPart {
    AlgorithmSection,
    ElementList,
    EquationSection
});

#[derive(CommonTraits!, Default)]
pub struct ElementList {
    pub node_data: NodeData,
    pub visibility: Visibility,
    pub elements: Vec<Element>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationSection {
    pub node_data: NodeData,
    pub initial: bool,
    pub equations: Vec<Equation>,
}

#[derive(CommonTraits!, Default)]
pub struct AlgorithmSection {
    pub node_data: NodeData,
    pub initial: bool,
    pub statements: Vec<Statement>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[allow(clippy::large_enum_variant)]
pub enum Element {
    #[default]
    Empty,
    ClassDefinition(ClassDefinition),
    ComponentClause(Vec<ComponentDeclaration>),
    ExtendsClause(ExtendsClause),
    ImportClause(ImportClause),
}

impl_debug_for_enum!(Element {
    ClassDefinition,
    ComponentClause,
    ExtendsClause,
    ImportClause,
});

#[derive(CommonTraits!, Default)]
pub struct ImportClause {
    pub node_data: NodeData,
    pub alias: String,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct ExtendsClause {
    pub node_data: NodeData,
    pub type_specifier: TypeSpecifier,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentDeclaration {
    pub node_data: NodeData,
    pub name: String,
    pub type_specifier: TypeSpecifier,
    pub flags: ElementFlags,
    pub connection: Connection,
    pub variability: Variability,
    pub causality: Causality,
    pub visibility: Visibility,
    pub array_subscripts: Vec<Subscript>,
    pub modification: Option<Modification>,
    pub condition_attribute: Option<Expression>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct ClassPrefixes {
    pub is_partial: bool,
    pub class_type: ClassType,
}

#[derive(CommonTraits!, Default)]
pub struct Declaration {
    pub node_data: NodeData,
    pub name: String,
    pub array_subscripts: Vec<Subscript>,
    pub modification: Option<Modification>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TypeSpecifier {
    pub local: bool,
    pub name: Name,
}

impl fmt::Debug for TypeSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.local {
            write!(f, ".{:?}", self.name)
        } else {
            write!(f, "{:?}", self.name)
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equations

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Equation {
    #[default]
    Empty,
    Connect(EquationConnect),
    For(EquationFor),
    If(EquationIf),
    Simple(EquationSimple),
}

impl_debug_for_enum!(Equation {
    Connect,
    For,
    If,
    Simple,
});

#[derive(CommonTraits!, Default)]
pub struct EquationSimple {
    pub node_data: NodeData,
    pub lhs: Expression,
    pub rhs: Expression,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<EquationIfBlock>,
    pub else_eqs: Vec<Equation>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationFor {
    pub node_data: NodeData,
    pub indices: Vec<ForIndex>,
    pub eqs: Vec<Equation>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationConnect {
    pub node_data: NodeData,
    pub lhs: ComponentReference,
    pub rhs: ComponentReference,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub eqs: Vec<Equation>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statements

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Statement {
    #[default]
    Empty,
    Assignment(StatementAssignment),
    Break(StatementBreak),
    For(StatementFor),
    If(StatementIf),
    Return(StatementReturn),
    While(StatementWhile),
}

impl_debug_for_enum!(Statement {
    Assignment,
    Break,
    For,
    If,
    Return,
    While,
});

#[derive(CommonTraits!, Default)]
pub struct StatementAssignment {
    pub node_data: NodeData,
    pub comp: ComponentReference,
    pub rhs: Expression,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<StatementIfBlock>,
    pub else_stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementFor {
    pub node_data: NodeData,
    pub indices: Vec<ForIndex>,
    pub stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementWhile {
    pub node_data: NodeData,
    pub cond: Expression,
    pub stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementBreak {
    pub node_data: NodeData,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementReturn {
    pub node_data: NodeData,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct StatementIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub stmts: Vec<Statement>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expressions

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Expression {
    #[default]
    Empty,
    Array(Array),
    Binary(Binary),
    Boolean(Boolean),
    FunctionCall(FunctionCall),
    If(ExpressionIf),
    Ref(ComponentReference),
    Unary(Unary),
    UnsignedInteger(UnsignedInteger),
    UnsignedReal(UnsignedReal),
}

impl_debug_for_enum!(Expression {
    Array,
    Binary,
    Boolean,
    FunctionCall,
    If,
    Ref,
    Unary,
    UnsignedInteger,
    UnsignedReal
});

#[derive(CommonTraits!, Default)]
pub struct Array {
    pub node_data: NodeData,
    pub args: Vec<Expression>,
}

#[derive(CommonTraits!, Default)]
pub struct Unary {
    pub node_data: NodeData,
    pub op: UnaryOp,
    pub rhs: Box<Expression>,
}

#[derive(CommonTraits!, Default)]
pub struct Binary {
    pub node_data: NodeData,
    pub op: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(CommonTraits!, Default)]
pub struct ExpressionIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<ExpressionIfBlock>,
    pub else_expr: Box<Option<Expression>>,
}

#[derive(CommonTraits!, Default)]
pub struct FunctionCall {
    pub node_data: NodeData,
    pub comp: ComponentReference,
    pub args: Vec<Expression>,
}

#[derive(CommonTraits!, Default)]
pub struct ExpressionIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub expr: Expression,
}

#[derive(CommonTraits!, Default)]
pub struct UnsignedInteger {
    pub node_data: NodeData,
    pub val: String,
}

#[derive(CommonTraits!, Default)]
pub struct UnsignedReal {
    pub node_data: NodeData,
    pub val: String,
}

#[derive(CommonTraits!, Default)]
pub struct Boolean {
    pub node_data: NodeData,
    pub val: bool,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ComponentReference {
    pub node_data: NodeData,
    pub local: bool,
    pub parts: Vec<RefPart>,
}

impl fmt::Debug for ComponentReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = self
            .parts
            .iter()
            .map(|part| format!("{:?}", part))
            .collect::<Vec<_>>()
            .join(".");
        write!(
            f,
            "ComponentReference {{{}{}, id: {}, span: {:?}}}",
            if self.local { "." } else { "" },
            parts,
            self.node_data.id,
            self.node_data.span,
        )
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RefPart {
    pub node_data: NodeData,
    pub name: String,
    pub array_subscripts: Vec<Subscript>,
}

impl fmt::Debug for RefPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.array_subscripts.is_empty() {
            write!(f, "{:?}", self.name)
        } else {
            write!(f, "{:?}{:?}", self.name, self.array_subscripts)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Subscript {
    #[default]
    Empty,
    Range(SubscriptRange),
    Expression(Expression),
}

impl_debug_for_enum!(Subscript { Range, Expression });

#[derive(CommonTraits!, Default)]
pub struct SubscriptRange {
    pub node_data: NodeData,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Modification

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Argument {
    #[default]
    Empty,
    Modification(ArgumentModification),
    Redeclaration(ArgumentRedeclaration),
    Replaceable(ArgumentReplaceable),
}

impl_debug_for_enum!(Argument {
    Modification,
    Redeclaration,
    Replaceable
});

#[derive(CommonTraits!, Default)]
pub struct ArgumentModification {
    pub node_data: NodeData,
    pub name: Name,
    pub each: bool,
    pub is_final: bool,
    pub modification: Option<Modification>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default)]
pub struct ArgumentRedeclaration {
    pub node_data: NodeData,
}

#[derive(CommonTraits!, Default)]
pub struct ArgumentReplaceable {
    pub node_data: NodeData,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Modification {
    #[default]
    Empty,
    Class(ModificationClass),
    Expression(ModExpr),
}

impl_debug_for_enum!(Modification { Class, Expression });

#[derive(CommonTraits!, Default)]
pub struct ModificationClass {
    pub node_data: NodeData,
    pub args: Vec<Argument>,
    pub expr: Option<ModExpr>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModExpr {
    #[default]
    Empty,
    Break(ModExprBreak),
    Expression(Expression),
}

impl_debug_for_enum!(ModExpr { Break, Expression });

#[derive(CommonTraits!, Default)]
pub struct ModExprBreak {
    pub node_data: NodeData,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Description {
    pub node_data: NodeData,
    pub strings: Vec<String>,
    pub annotation: Vec<Argument>,
}

impl fmt::Debug for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.strings.join(" "), self.annotation)
    }
}

#[derive(CommonTraits!, Default)]
pub struct TypePrefix {
    pub connection: Connection,
    pub variability: Variability,
    pub causality: Causality,
}

#[derive(CommonTraits!, Default)]
pub struct ForIndex {
    pub node_data: NodeData,
    pub ident: String,
    pub in_expr: Option<Expression>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Simple Terminals

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ElementFlags {
    pub replaceable: bool,
    pub redeclare: bool,
    pub is_final: bool,
    pub inner: bool,
    pub outer: bool,
}

impl fmt::Debug for ElementFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = [
            ("replaceable", self.replaceable),
            ("redeclare", self.redeclare),
            ("is_final", self.is_final),
            ("inner", self.inner),
            ("outer", self.outer),
        ]
        .iter()
        .filter_map(|&(flag, is_set)| if is_set { Some(flag) } else { None })
        .collect::<Vec<_>>(); // Collect names of true flags into a vector
        write!(f, "[{}]", flags.join(", "))
    }
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
    Constant,
    Continuous,
    Discrete,
    Parameter,
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
    ElemNegative,
    ElemPositive,
    Negative,
    Not,
    Paren,
    Positive,
}

#[derive(CommonTraits!, Default)]
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

#[derive(CommonTraits!, Default)]
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DescriptionString {
    pub parts: Vec<String>,
}

impl fmt::Debug for DescriptionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join(" "))
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Name {
    pub parts: Vec<String>,
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
