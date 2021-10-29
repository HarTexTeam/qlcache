//! # Module `constraints`
//!
//! This module implements constraints for the query language.

use crate::{
    error::{
        QlError,
        QlResult
    },
    ColumnValue
};

/// # Trait `ComputableConstraint`
///
/// A computable constraint, returns whether the constraint is satisfied upon computation.
pub trait ComputableConstraint {
    /// # Trait Method `ComputableConstraint::compute`
    ///
    /// Computes this constraint and returns whether the constraint is satisfied.
    fn compute(&self) -> bool;
}

/// # Struct `Constraint`
///
/// A constraint for data queries.
///
/// ## Example
///
/// To construct a constraint, that the value of field `FieldName`, of type `I32`, has the
/// value `2138`:
///
/// ```
/// use qlcache::{
///     ql::constraints::{
///         Constraint,
///         ConstraintOp
///     },
///     ColumnValue
/// };
///
/// let constraint = Constraint::builder()
///     .field_name(String::from("FieldName"))
///     .op(ConstraintOp::Eq)
///     .value(ColumnValue::I32(2138))
///     .build()
///     .unwrap();
/// ```
#[allow(dead_code)]
#[derive(Clone)]
pub struct Constraint {
    pub(crate) field_name: String,
    pub(crate) op: ConstraintOp,
    pub(crate) value: ColumnValue
}

impl Constraint {
    /// # Static Method `Constraint::builder`
    ///
    /// Returns a builder for a `Constraint`.
    #[must_use]
    pub fn builder() -> ConstraintBuilder {
        ConstraintBuilder {
            field_name: None,
            op: None,
            value: None
        }
    }
}

impl ComputableConstraint for Constraint {
    fn compute(&self) -> bool {
        todo!()
    }
}

/// # Struct `ConstraintBuilder`
///
/// A builder for a `Constraint`.
#[derive(Clone)]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn value(mut self, value: ColumnValue) -> Self {
        self.value.replace(value);
        self
    }

    /// # Instance Method `ConstraintBuilder::build`
    ///
    /// Consumes the builder and returns a `Constraint`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the fields required is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build(self) -> QlResult<Constraint> {
        if self.field_name.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("ConstraintBuilder.field_name")
            });
        }

        if self.op.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("ConstraintBuilder.op")
            });
        }

        if self.value.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("ConstraintBuilder.value")
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

/// # Struct `AndConstraint`
///
/// An "and" constraint.
pub struct AndConstraint {
    pub(crate) left: Box<dyn ComputableConstraint>,
    pub(crate) right: Box<dyn ComputableConstraint>
}

impl AndConstraint {
    /// # Constructor `AndConstraint::new`
    ///
    /// Constructs a new `And` constraint.
    #[must_use]
    pub fn new(left: Box<dyn ComputableConstraint>, right: Box<dyn ComputableConstraint>) -> Self {
        Self {
            left,
            right
        }
    }
}

impl ComputableConstraint for AndConstraint {
    fn compute(&self) -> bool {
        self.left.compute() && self.right.compute()
    }
}

/// # Struct `OrConstraint`
///
/// An "or" constraint.
pub struct OrConstraint {
    pub(crate) left: Box<dyn ComputableConstraint>,
    pub(crate) right: Box<dyn ComputableConstraint>
}

impl OrConstraint {
    /// # Constructor `OrConstraint::new`
    ///
    /// Constructs a new `Or` constraint.
    #[must_use]
    pub fn new(left: Box<dyn ComputableConstraint>, right: Box<dyn ComputableConstraint>) -> Self {
        Self {
            left,
            right
        }
    }
}

impl ComputableConstraint for OrConstraint {
    fn compute(&self) -> bool {
        self.left.compute() || self.right.compute()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AndConstraint,
        ComputableConstraint,
        Constraint,
        ConstraintBuilder,
        ConstraintOp,
        OrConstraint
    };

    static_assertions::assert_impl_all!(AndConstraint: ComputableConstraint, Send, Sync);
    static_assertions::assert_impl_all!(Constraint: Clone, ComputableConstraint, Send, Sync);
    static_assertions::assert_impl_all!(ConstraintBuilder: Clone, Send, Sync);
    static_assertions::assert_impl_all!(ConstraintOp: Clone, Eq, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(OrConstraint: ComputableConstraint, Send, Sync);
}
