//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

use crate::{
    error::{
        QlError,
        QlResult
    },
    ql::constraints::{
        AndConstraint,
        BoxedConstraint,
        OrConstraint
    }
};

#[allow(dead_code)]
pub(crate) struct Select {
    pub(crate) table_name: String,
    pub(crate) scope: SelectScope,
    pub(crate) constraint: Option<BoxedConstraint>
}

impl Select {
    #[must_use]
    pub(crate) fn builder() -> SelectBuilder {
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
///
/// ## Examples
///
/// Case 1: Basic `SELECT` queries
/// - select *everything*
/// ```
/// use qlcache::ql::{
///     select::SelectScope,
///     QueryBuilder
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .build()
///     .unwrap();
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct SelectBuilder {
    pub(crate) table_name: Option<String>,
    pub(crate) scope: Option<SelectScope>,
    pub(crate) constraint: Option<BoxedConstraint>
}

impl SelectBuilder {
    /// # Instance Method `SelectBuilder::table_name`
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

    /// # Instance Method `SelectBuilder::scope`
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
    #[must_use]
    pub fn constraint(mut self, constraint: BoxedConstraint) -> Self {
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
    pub fn and(mut self, constraint: BoxedConstraint) -> QlResult<Self> {
        if self.constraint.is_none() {
            return Err(QlError::NoFirstConstraintFoundBeforeAndOr);
        }

        let exist_constraint = self.constraint.take().unwrap();

        self.constraint
            .replace(Box::new(AndConstraint::new(exist_constraint, constraint)));
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
    pub fn or(mut self, constraint: BoxedConstraint) -> QlResult<Self> {
        if self.constraint.is_none() {
            return Err(QlError::NoFirstConstraintFoundBeforeAndOr);
        }

        let exist_constraint = self.constraint.take().unwrap();

        self.constraint
            .replace(Box::new(OrConstraint::new(exist_constraint, constraint)));
        Ok(self)
    }

    /// # Instance Method `SelectBuidler::build`
    ///
    /// Consumes the builder and returns a `Select`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the fields required is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build(self) -> QlResult<Select> {
        if self.table_name.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("SelectBuilder.table_name")
            });
        }

        if self.scope.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("SelectBuilder.scope")
            });
        }

        Ok(Select {
            table_name: self.table_name.unwrap(),
            scope: self.scope.unwrap(),
            constraint: self.constraint
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}

#[cfg(test)]
mod tests {
    use super::{
        Select,
        SelectBuilder
    };

    static_assertions::assert_impl_all!(Select: Send, Sync);
    static_assertions::assert_impl_all!(SelectBuilder: Send, Sync);
}
