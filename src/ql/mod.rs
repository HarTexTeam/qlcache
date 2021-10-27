//! # Module `ql`
//!
//! This module implements the query language.

use crate::{
    error::{
        QlError,
        QlResult
    },
    ql::select::{
        Select,
        SelectBuilder
    },
    ColumnValue
};

pub mod select;

/// # Trait `QueryKind`
///
/// The super trait for various query kinds.
pub trait QueryKind {}

/// # Struct `Query`
///
/// A query.
pub struct Query<T: QueryKind> {
    pub(crate) query: T
}

impl<T: QueryKind> Query<T> {
    pub fn builder() -> QueryBuilder {
        QueryBuilder {}
    }
}

/// # Struct `QueryBuilder`
///
/// The builder for a `Query`.
pub struct QueryBuilder;

impl QueryBuilder {
    /// # Instance Method `QueryBuilder::select`
    ///
    /// Returns a builder for building a `SELECT` query.
    pub fn select() -> SelectBuilder {
        Select::builder()
    }
}

/// # Struct `Constraint`
///
/// A constraint for data queries.
#[derive(Clone, Eq, PartialEq)]
pub struct Constraint {
    pub(crate) field_name: String,
    pub(crate) op: ConstraintOp,
    pub(crate) value: ColumnValue
}

impl Constraint {
    /// # Instance Method `Constraint::builder`
    ///
    /// Returns a builder for a `Constraint`
    pub fn builder() -> ConstraintBuilder {
        ConstraintBuilder {
            field_name: None,
            op: None,
            value: None
        }
    }
}

/// # Struct `ConstraintBuilder`
///
/// A builder for a `Constraint`.
#[derive(Clone, Eq, PartialEq)]
pub struct ConstraintBuilder {
    pub(crate) field_name: Option<String>,
    pub(crate) op: Option<ConstraintOp>,
    pub(crate) value: Option<ColumnValue>
}

impl ConstraintBuilder {
    /// # Instance Method `ConstraintBuilder::field_name`
    ///
    /// Sets the field name for the constraint.
    ///
    /// ## Parameters
    /// - `field_name`, type `String`; the field name to set
    pub fn field_name(mut self, field_name: String) -> Self {
        self.field_name.replace(field_name);
        self
    }

    /// # Instance Method `ConstraintBuilder::op`
    ///
    /// Sets the "constraint relational operator" for the constraint.
    ///
    /// ## Parameters
    /// - `op`, type `ConstraintOp`; the "constraint relational operator" name to set
    pub fn op(mut self, op: ConstraintOp) -> Self {
        self.op.replace(op);
        self
    }

    /// # Instance Method `ConstraintBuilder::value`
    ///
    /// Sets the value to check against for the constraint.
    ///
    /// ## Parameters
    /// - `op`, type `ConstraintOp`; the "constraint relational operator" name to set
    pub fn value(mut self, value: ColumnValue) -> Self {
        self.value.replace(value);
        self
    }

    /// # Instance Method `ConstraintBuilder::build`
    ///
    /// Consumes the builder and returns a `Constraint`.
    pub fn build(self) -> QlResult<Constraint> {
        if self.field_name.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("Constraint.field_name")
            });
        }

        if self.op.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("Constraint.op")
            });
        }

        if self.value.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("Constraint.value")
            });
        }

        Ok(Constraint {
            field_name: self.field_name.unwrap(),
            op: self.op.unwrap(),
            value: self.value.unwrap()
        })
    }
}

/// # Enumeration `ConstraintOp`
///
/// Represents a "constraint relational operator".
#[derive(Clone, Eq, PartialEq)]
pub enum ConstraintOp {
    /// # Enumeration Variant `ConstraintOp::Eq`
    ///
    /// `=`
    Eq,

    /// # Enumeration Variant `ConstraintOp::Lt`
    ///
    /// `<`
    Lt,

    /// # Enumeration Variant `ConstraintOp::Gt`
    ///
    /// `>`
    Gt,

    /// # Enumeration Variant `ConstraintOp::Le`
    ///
    /// `<=`
    Le,

    /// # Enumeration Variant `ConstraintOp::Ge`
    ///
    /// `>=`
    Ge
}
