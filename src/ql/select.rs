//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

use crate::ql::constraints::ComputableConstraint;

pub(crate) struct Select<C>
where
    C: ComputableConstraint;

impl<C> Select<C>
where
    C: ComputableConstraint
{
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
    C: ComputableConstraint
{
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
}

#[allow(clippy::module_name_repetitions)]
pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}
