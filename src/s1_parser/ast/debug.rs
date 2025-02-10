//! This module implements the `fmt::Debug` trait for the AST types.

use super::node::*;
use super::part::*;
use std::fmt;

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

impl fmt::Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeData {{id: {:?}, span: {:?}}}", self.id, self.span)
    }
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

impl fmt::Debug for TypeSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.local {
            write!(f, ".{:?}", self.name)
        } else {
            write!(f, "{:?}", self.name)
        }
    }
}

impl_debug_for_enum!(Equation {
    Connect,
    For,
    If,
    Simple,
});

impl_debug_for_enum!(Statement {
    Assignment,
    Break,
    For,
    If,
    Return,
    While,
});

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

impl fmt::Debug for RefPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.array_subscripts.is_empty() {
            write!(f, "{:?}", self.name)
        } else {
            write!(f, "{:?}{:?}", self.name, self.array_subscripts)
        }
    }
}

impl_debug_for_enum!(Subscript { Range, Expression });

impl_debug_for_enum!(Argument {
    Modification,
    Redeclaration,
    Replaceable
});

impl_debug_for_enum!(Modification { Class, Expression });

impl_debug_for_enum!(ModExpr { Break, Expression });

impl fmt::Debug for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.strings.join(" "), self.annotation)
    }
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

impl fmt::Debug for DescriptionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join(" "))
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
