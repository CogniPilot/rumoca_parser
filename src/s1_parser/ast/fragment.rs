//! This module contains AST fragments
//!
//! The fragments do not appear in the AST but are used
//! by the parser to build the AST.

use super::node::*;
use super::part::*;
use serde::{Deserialize, Serialize};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Class Level Fragments

#[derive(CommonTraits!, Default)]
pub enum CompositionPart {
    #[default]
    Empty,
    AlgorithmSection(AlgorithmSection),
    ElementList(ElementList),
    EquationSection(EquationSection),
}

#[derive(CommonTraits!, Default)]
pub struct ElementList {
    pub visibility: Visibility,
    pub elements: Vec<Element>,
}

#[derive(CommonTraits!, Default)]
pub struct EquationSection {
    pub initial: bool,
    pub equations: Vec<Equation>,
}

#[derive(CommonTraits!, Default)]
pub struct AlgorithmSection {
    pub initial: bool,
    pub statements: Vec<Statement>,
}

#[derive(CommonTraits!, Default)]
#[allow(clippy::large_enum_variant)]
pub enum Element {
    #[default]
    Empty,
    ClassDefinition(ClassDefinition),
    ComponentClause(Vec<ComponentDeclaration>),
    ExtendsClause(ExtendsClause),
    ImportClause(ImportClause),
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
    pub name: String,
    pub description: DescriptionString,
    pub composition: Vec<CompositionPart>,
    pub name_end: String,
}

#[derive(CommonTraits!, Default)]
pub struct ClassSpecifierExtends {
    pub name: String,
    pub modification: Vec<Argument>,
    pub description: DescriptionString,
    pub composition: Vec<CompositionPart>,
    pub name_end: String,
}

#[derive(CommonTraits!, Default)]
pub struct ClassPrefixes {
    pub is_partial: bool,
    pub class_type: ClassType,
}

#[derive(CommonTraits!, Default)]
pub struct Declaration {
    pub name: String,
    pub array_subscripts: Vec<Subscript>,
    pub modification: Option<Modification>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Equations

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Statements

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Expressions

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Common

#[derive(CommonTraits!, Default)]
pub struct TypePrefix {
    pub connection: Connection,
    pub variability: Variability,
    pub causality: Causality,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Simple Terminals
