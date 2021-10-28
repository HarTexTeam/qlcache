//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

use std::marker::PhantomData;

use crate::{
    error::{
        QlError,
        QlResult
    },
    ql::constraints::{
        AndConstraint,
        ComputableConstraint,
        OrConstraint
    }
};

pub(crate) struct Select<C>
where
    C: ComputableConstraint {
    phantom: PhantomData<C>
}

impl<C> Select<C>
where
    C: ComputableConstraint {
    #[must_use]
    pub(crate) fn builder() -> SelectBuilder<C> {
        SelectBuilder {
            table_name: None,
            scope: None,
            constraint: None
        }
    }
}

/// # Struct `SelectBuilder`
///
/// A builder for a `Select`.
#[allow(clippy::module_name_repetitions)]
pub struct SelectBuilder<C>
where
    C: ComputableConstraint {
    pub(crate) table_name: Option<String>,
    pub(crate) scope: Option<SelectScope>,
    pub(crate) constraint: Option<C>
}

impl<C> SelectBuilder<C>
where
    C: ComputableConstraint {
    /// # Instance Method `SelectBuilder::field_name`
    ///
    /// Sets the table name for the selection.
    ///
    /// ## Parameters
    /// - `table_name`, type `String`; the table name to set
    #[must_use]
    pub fn table_name(mut self, table_name: String) -> Self {
        self.table_name.replace(table_name);
        self
    }

    /// # Instance Method `SelectBuilder::field_name`
    ///
    /// Sets the selection scope for the selection.
    ///
    /// ## Parameters
    /// - `scope`, type `SelectScope`; the selection scope to set
    #[must_use]
    pub fn scope(mut self, scope: SelectScope) -> Self {
        self.scope.replace(scope);
        self
    }

    /// # Instance Method `SelectBuilder::constraint`
    ///
    /// Sets a constraint for the selection
    ///
    /// ## Parameters
    /// - `constraint`, type `C`; the constraint to add
    pub fn constraint(mut self, constraint: C) -> Self {
        self.constraint.replace(constraint);
        self
    }

    /// # Instance Method `SelectBuilder::and`
    ///
    /// Adds an "and" constraint, with operands the previously added constraint,
    /// as well as the constraint passed to the `constraint` parameter.
    ///
    /// ## Parameters
    /// - `constraint`, type `C`; the constraint to add
    ///
    /// ## Errors
    ///
    /// Returns `NoFirstConstraintFoundBeforeAndOr` if `constraint` in the `SelectBuilder`
    /// structure is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn and(mut self, constraint: C) -> QlResult<Self> {
        if self.constraint.is_none() {
            return Err(QlError::NoFirstConstraintFoundBeforeAndOr);
        }

        self.constraint.replace(
            AndConstraint::new(self.constraint.unwrap(), constraint)
        );
        Ok(self)
    }

    /// # Instance Method `SelectBuilder::or`
    ///
    /// Adds an "or" constraint, with operands the previously added constraint,
    /// as well as the constraint passed to the `constraint` parameter.
    ///
    /// ## Parameters
    /// - `constraint`, type `C`; the constraint to add
    ///
    /// ## Errors
    ///
    /// Returns `NoFirstConstraintFoundBeforeAndOr` if `constraint` in the `SelectBuilder`
    /// structure is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn or(mut self, constraint: C) -> QlResult<Self> {
        if self.constraint.is_none() {
            return Err(QlError::NoFirstConstraintFoundBeforeAndOr);
        }

        self.constraint.replace(
            OrConstraint::new(self.constraint.unwrap(), constraint)
        );
        Ok(self)
    }
}

#[allow(clippy::module_name_repetitions)]
pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}
