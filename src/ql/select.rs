//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

use std::marker::PhantomData;

use crate::{
    error::{
        QlError,
        QlResult,
        QueryError
    },
    ql::{
        constraints::{
            AndConstraint,
            BoxedConstraint,
            OrConstraint
        },
        sortby::SortBy,
        Query,
        QueryAs,
        QueryAsType,
        QueryRow
    },
    CacheTableRow,
    FromRow,
    QlCache
};

/// # Struct `Select`
///
/// A `SELECT` query.
#[allow(dead_code)]
pub struct Select {
    pub(crate) table_name: String,
    pub(crate) scope: SelectScope,
    pub(crate) constraint: Option<BoxedConstraint>,
    pub(crate) sort_by: Option<SortBy>
}

impl Select {
    #[must_use]
    pub(crate) fn builder() -> SelectBuilder {
        SelectBuilder {
            table_name: None,
            scope: None,
            constraint: None,
            sort_by: None
        }
    }
}

impl QueryRow for Select {
    fn execute(self, cache: &QlCache) -> QlResult<Vec<CacheTableRow>> {
        let mut name = self.table_name.split(' ');
        let all = if name.clone().count() == 1 {
            // if the length of the split is only 1, we select from the PUBLIC schema.
            let name = name.next().unwrap();

            let entry = cache.cache.get("PUBLIC").unwrap();
            let public = entry.value();
            let table = if let Some(entry) = public.tables.get(name) {
                entry.value().clone()
            }
            else {
                return Err(QlError::QueryError(QueryError::RelationDoesNotExist {
                    name: name.to_string()
                }));
            };

            table.rows
        }
        else {
            // if the length of the split is 2, we select from the schema specified by the first
            // segment, and the table specified by the second segment.
            let schema_name = name.next().unwrap();
            let schema = if let Some(entry) = cache.cache.get(schema_name) {
                entry.value().clone()
            }
            else {
                return Err(QlError::QueryError(QueryError::RelationDoesNotExist {
                    name: self.table_name
                }));
            };

            let table_name = name.next().unwrap();
            let table = if let Some(entry) = schema.tables.get(table_name) {
                entry.value().clone()
            }
            else {
                return Err(QlError::QueryError(QueryError::RelationDoesNotExist {
                    name: self.table_name
                }));
            };

            table.rows
        };
        let rows: Vec<CacheTableRow> = all.into_iter().map(|(_, row)| row).collect();

        if self.scope == SelectScope::Everything && self.constraint.is_none() {
            // no constraints
            return Ok(rows);
        }
        else {}

        if let SelectScope::Fields(_) = self.scope && self.constraint.is_none() {
            // no constraints
        }
        else {}

        todo!()
    }
}

impl<'row, T: FromRow<'row>> QueryAsType<'row, T> for Select {
    fn execute_as(self, _: &QlCache) -> QlResult<T> {
        todo!()
    }
}

/// # Struct `SelectBuilder`
///
/// A builder for a `Select`, constructs a `SELECT` query.
///
/// ## Examples
///
/// Case 1: Basic `SELECT` queries
/// - select *everything* (`SELECT * FROM TableName`):
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
///
/// - select *specific fields* (`SELECT Field1 FROM TableName`):
/// ```
/// use qlcache::ql::{
///     select::SelectScope,
///     QueryBuilder
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Fields(vec![String::from("Field1")]))
///     .build()
///     .unwrap();
/// ```
///
/// Case 2: Filtering
/// - *one* constraint (`SELECT * FROM TableName WHERE Field1 = "XE"`)
/// ```
/// use qlcache::{
///     ql::{
///         constraints::{
///             Constraint,
///             ConstraintOp
///         },
///         select::SelectScope,
///         QueryBuilder
///     },
///     ColumnValue
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .constraint(Box::new(Constraint::builder()
///         .field_name(String::from("Field1"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::String(String::from("XE")))
///         .build()
///         .unwrap()))
///     .build();
/// ```
///
/// - *and* constraints (`SELECT * FROM TableName WHERE Field1 = "XE" AND Field2 = 2138`):
/// ```
/// use qlcache::{
///     ql::{
///         constraints::{
///             Constraint,
///             ConstraintOp
///         },
///         select::SelectScope,
///         QueryBuilder
///     },
///     ColumnValue
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .constraint(Box::new(Constraint::builder()
///         .field_name(String::from("Field1"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::String(String::from("XE")))
///         .build()
///         .unwrap()
///     ))
///     .and(Box::new(Constraint::builder()
///         .field_name(String::from("Field2"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::I32(2138))
///         .build()
///         .unwrap()
///     ))
///     .unwrap()
///     .build();
/// ```
///
/// - *not* constraints (`SELECT * FROM TableName WHERE NOT Field1 = "XE`):
/// ```
/// use qlcache::{
///     ql::{
///         constraints::{
///             Constraint,
///             ConstraintOp,
///             NotConstraint
///         },
///         select::SelectScope,
///         QueryBuilder
///     },
///     ColumnValue
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .constraint(Box::new(NotConstraint::new(Box::new(Constraint::builder()
///         .field_name(String::from("Field1"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::String(String::from("XE")))
///         .build()
///         .unwrap()))
///      ))
///      .build()
///      .unwrap();
/// ```
///
/// - *or* constraints (`SELECT * FROM TableName WHERE Field1 = "XE" OR Field2 = 2138`):
/// ```
/// use qlcache::{
///     ql::{
///         constraints::{
///             Constraint,
///             ConstraintOp
///         },
///         select::SelectScope,
///         QueryBuilder
///     },
///     ColumnValue
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .constraint(Box::new(Constraint::builder()
///         .field_name(String::from("Field1"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::String(String::from("XE")))
///         .build()
///         .unwrap()
///     ))
///     .or(Box::new(Constraint::builder()
///         .field_name(String::from("Field2"))
///         .op(ConstraintOp::Eq)
///         .value(ColumnValue::I32(2138))
///         .build()
///         .unwrap()
///     ))
///     .unwrap()
///     .build();
/// ```
///
/// Case 3: Sort Selected Items
/// - in *descending* order (`SELECT * FROM TableName SORT BY Field1 DESCENDING`):
/// ```
/// use qlcache::ql::{
///     select::SelectScope,
///     sortby::{
///         SortBy,
///         SortOrdering
///     },
///     QueryBuilder
/// };
///
/// let select = QueryBuilder::select()
///     .table_name(String::from("TableName"))
///     .scope(SelectScope::Everything)
///     .sort_by(SortBy::new(
///         vec![String::from("Field1")],
///         Some(SortOrdering::Descending)).unwrap())
///     .build()
///     .unwrap();
/// ```
///
/// Pass in `SortOrdering::Ascending` if you want to sort the items in ascending order instead.
#[allow(clippy::module_name_repetitions)]
pub struct SelectBuilder {
    pub(crate) table_name: Option<String>,
    pub(crate) scope: Option<SelectScope>,
    pub(crate) constraint: Option<BoxedConstraint>,
    pub(crate) sort_by: Option<SortBy>
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

    /// # Instance Method `SelectBuilder::or`
    ///
    /// Configures how results are sorted.
    ///
    /// ## Parameters
    /// - `sort_by`, type `SortBy`: the sort configuration
    #[must_use]
    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by.replace(sort_by);
        self
    }

    /// # Instance Method `SelectBuilder::build`
    ///
    /// Consumes the builder and returns a `Query<Select>`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the fields required is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build(self) -> QlResult<Query<Select>> {
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

        Ok(Query {
            query: Select {
                table_name: self.table_name.unwrap(),
                scope: self.scope.unwrap(),
                constraint: self.constraint,
                sort_by: self.sort_by
            }
        })
    }

    /// # Instance Method `SelectBuilder::build_as`
    ///
    /// Consumes the builder and returns a `QueryAs<Select, T>`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the fields required is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build_as<'row, T: FromRow<'row>>(self) -> QlResult<QueryAs<'row, Select, T>> {
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

        Ok(QueryAs {
            query: Select {
                table_name: self.table_name.unwrap(),
                scope: self.scope.unwrap(),
                constraint: self.constraint,
                sort_by: self.sort_by
            },
            phantom: PhantomData
        })
    }
}

/// # Enumeration `SelectScope`
///
/// The scope for the `SELECT` query.
#[allow(clippy::module_name_repetitions)]
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Clone, Eq, PartialEq)]
pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}

#[cfg(test)]
mod tests {
    use super::{
        QueryAsType,
        QueryRow,
        Select,
        SelectBuilder,
        SelectScope
    };
    use crate::{
        error::QlResult,
        CacheTableRow,
        FromRow
    };

    struct Dummy;

    impl<'row> FromRow<'row> for Dummy {
        fn from_row(_: &'row CacheTableRow) -> QlResult<Self> {
            todo!()
        }
    }

    const _: fn() = || {
        fn assert_impl_all<'row, T: ?Sized + QueryAsType<'row, Dummy> + QueryRow + Send + Sync>() {}
        assert_impl_all::<Select>();
    };

    static_assertions::assert_impl_all!(SelectBuilder: Send, Sync);
    static_assertions::assert_impl_all!(SelectScope: Clone, Eq, PartialEq, Send, Sync);
}
