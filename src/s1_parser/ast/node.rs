//! This module contains AST nodes.
//!
//! Nodes are used are annotated for with span, and id information.
//! They are meant to be used for visitor patterns and tree annotation.
//! Note that enums are also considered nodes if their variants are all nodes.

use super::part::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Level Nodes
#[derive(CommonTraits!, Default, Debug)]
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

#[derive(CommonTraits!, Default, Debug)]
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

#[derive(CommonTraits!, Default, Debug)]
pub struct ImportClause {
    pub node_data: NodeData,
    pub alias: String,
    pub name: Name,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ExtendsClause {
    pub node_data: NodeData,
    pub type_specifier: TypeSpecifier,
}

#[derive(CommonTraits!, Default, Debug)]
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
pub struct TypeSpecifier {
    pub node_data: NodeData,
    pub local: bool,
    pub name: Name,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equations

#[derive(CommonTraits!, Default)]
pub enum Equation {
    #[default]
    Empty,
    Connect(EquationConnect),
    For(EquationFor),
    If(EquationIf),
    Simple(EquationSimple),
}

#[derive(CommonTraits!, Default, Debug)]
pub struct EquationSimple {
    pub node_data: NodeData,
    pub lhs: Expression,
    pub rhs: Expression,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct EquationIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<EquationIfBlock>,
    pub else_eqs: Vec<Equation>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct EquationFor {
    pub node_data: NodeData,
    pub indices: Vec<ForIndex>,
    pub eqs: Vec<Equation>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct EquationConnect {
    pub node_data: NodeData,
    pub lhs: ComponentReference,
    pub rhs: ComponentReference,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct EquationIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub eqs: Vec<Equation>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statements

#[derive(CommonTraits!, Default)]
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

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementAssignment {
    pub node_data: NodeData,
    pub comp: ComponentReference,
    pub rhs: Expression,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<StatementIfBlock>,
    pub else_stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementFor {
    pub node_data: NodeData,
    pub indices: Vec<ForIndex>,
    pub stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementWhile {
    pub node_data: NodeData,
    pub cond: Expression,
    pub stmts: Vec<Statement>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementBreak {
    pub node_data: NodeData,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementReturn {
    pub node_data: NodeData,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct StatementIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub stmts: Vec<Statement>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expressions

#[derive(CommonTraits!, Default)]
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

#[derive(CommonTraits!, Default, Debug)]
pub struct Array {
    pub node_data: NodeData,
    pub args: Vec<Expression>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct Unary {
    pub node_data: NodeData,
    pub op: UnaryOp,
    pub rhs: Box<Expression>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct Binary {
    pub node_data: NodeData,
    pub op: BinaryOp,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ExpressionIf {
    pub node_data: NodeData,
    pub if_blocks: Vec<ExpressionIfBlock>,
    pub else_expr: Box<Option<Expression>>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct FunctionCall {
    pub node_data: NodeData,
    pub comp: ComponentReference,
    pub args: Vec<Expression>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ExpressionIfBlock {
    pub node_data: NodeData,
    pub cond: Expression,
    pub expr: Expression,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct UnsignedInteger {
    pub node_data: NodeData,
    pub val: String,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct UnsignedReal {
    pub node_data: NodeData,
    pub val: String,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct Boolean {
    pub node_data: NodeData,
    pub val: bool,
}

#[derive(CommonTraits!, Default)]
pub struct ComponentReference {
    pub node_data: NodeData,
    pub local: bool,
    pub parts: Vec<RefPart>,
}

#[derive(CommonTraits!, Default)]
pub struct RefPart {
    pub node_data: NodeData,
    pub name: String,
    pub array_subscripts: Vec<Subscript>,
}

#[derive(CommonTraits!, Default)]
pub enum Subscript {
    #[default]
    Empty,
    Range(SubscriptRange),
    Expression(Expression),
}

#[derive(CommonTraits!, Default, Debug)]
pub struct SubscriptRange {
    pub node_data: NodeData,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Modification

#[derive(CommonTraits!, Default)]
pub enum Argument {
    #[default]
    Empty,
    Modification(ArgumentModification),
    Redeclaration(ArgumentRedeclaration),
    Replaceable(ArgumentReplaceable),
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ArgumentModification {
    pub node_data: NodeData,
    pub name: Name,
    pub each: bool,
    pub is_final: bool,
    pub modification: Option<Modification>,
    pub description: Option<Description>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ArgumentRedeclaration {
    pub node_data: NodeData,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ArgumentReplaceable {
    pub node_data: NodeData,
}

#[derive(CommonTraits!, Default)]
pub enum Modification {
    #[default]
    Empty,
    Class(ModificationClass),
    Expression(ModExpr),
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ModificationClass {
    pub node_data: NodeData,
    pub args: Vec<Argument>,
    pub expr: Option<ModExpr>,
}

#[derive(CommonTraits!, Default)]
pub enum ModExpr {
    #[default]
    Empty,
    Break(ModExprBreak),
    Expression(Expression),
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ModExprBreak {
    pub node_data: NodeData,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common

#[derive(CommonTraits!, Default)]
pub struct Description {
    pub node_data: NodeData,
    pub strings: Vec<String>,
    pub annotation: Vec<Argument>,
}

#[derive(CommonTraits!, Default, Debug)]
pub struct ForIndex {
    pub node_data: NodeData,
    pub ident: String,
    pub in_expr: Option<Expression>,
}
