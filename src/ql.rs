//! # Module `ql`
//!
//! This module implements the query language.

use crate::ColumnValue;

/// # Enumeration `Command`
///
/// An enumeration representing various commands of the language.
#[derive(Clone, PartialEq)]
pub enum Command {
    Select(Select)
}

/// # Enumeration `Constraint`
///
/// An enumeration representing a constraint for the query.
#[derive(Clone, PartialEq)]
pub enum Constraint {
    Equality {
        expected: ColumnValue
    },

    Relational {
        larger_than: Option<ColumnValue>,
        smaller_than: Option<ColumnValue>
    },

    Range {
        upper_limit: Option<ColumnValue>,
        lower_limit: Option<ColumnValue>
    },

    NotNull
}

/// # Struct `Select`
///
/// Represents a `SELECT` command.
#[derive(Clone, PartialEq)]
pub struct Select {
    values: Values,
    constraints: Option<Vec<Constraint>>
}

/// # Enumeration `Values`
///
/// An enumeration representing the values to query.
#[derive(Clone, PartialEq)]
pub enum Values {
    Everything,
    Columns(Vec<String>)
}

#[cfg(test)]
mod tests {
    use super::{
        Command,
        Constraint,
        Select,
        Values
    };

    static_assertions::assert_impl_all!(Command: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Constraint: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Select: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Values: Clone, PartialEq, Send, Sync);
}
