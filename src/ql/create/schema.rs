//! # Module `schema`
//!
//! This module implements the `CREATE SCHEMA` query.

use dashmap::DashMap;

use crate::{
    error::{
        QlError,
        QlResult,
        QueryError
    },
    ql::{
        Query,
        QueryKind
    },
    CacheSchema,
    QlCache
};

/// # Struct `CreateSchema`
///
/// A `CREATE SCHEMA` query.
#[allow(clippy::module_name_repetitions, dead_code)]
pub struct CreateSchema {
    pub(crate) name: String,
    pub(crate) if_not_exist: bool
}

impl QueryKind for CreateSchema {
    type ResultType<'row> = ();

    fn execute(self, cache: &QlCache) -> QlResult<Self::ResultType<'_>> {
        if cache.cache.contains_key(&self.name) {
            if !self.if_not_exist {
                return Err(QlError::QueryError(QueryError::RelationAlreadyExists {
                    name: self.name
                }));
            }

            return Ok(());
        }

        cache.cache.insert(
            self.name.clone(),
            CacheSchema {
                name: self.name,
                tables: DashMap::new()
            }
        );

        Ok(())
    }
}

/// # Struct `CreateSchemaBuilder`
///
/// A builder for a `CreateSchema`, constructs a `CREATE SCHEMA` query.
///
/// ## Examples
/// - `CREATE SCHEMA SchemaName`
/// ```
/// use qlcache::ql::QueryBuilder;
///
/// let create_schema = QueryBuilder::create()
///     .schema()
///     .name(String::from("SchemaName"))
///     .build()
///     .unwrap();
/// ```
///
/// - `CREATE SCHEMA IF NOT EXIST SchemaName`
/// ```
/// use qlcache::ql::QueryBuilder;
///
/// let create_schema = QueryBuilder::create()
///     .schema()
///     .if_not_exist()
///     .name(String::from("SchemaName"))
///     .build();
/// ```
pub struct CreateSchemaBuilder {
    pub(crate) name: Option<String>,
    pub(crate) if_not_exist: bool
}

impl CreateSchemaBuilder {
    /// # Instance Method `CreateSchemaBuilder::name`
    ///
    /// Sets the name of the schema to create.
    ///
    /// ## Parameters
    /// - `name`, type `String`; the name of the schema
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name.replace(name);
        self
    }

    /// # Instance Method `CreateSchemaBuilder::if_not_exist`
    ///
    /// Sets the table to be created if it does not exist yet, otherwise does nothing.
    #[must_use]
    pub fn if_not_exist(mut self) -> Self {
        self.if_not_exist = true;
        self
    }

    /// # Instance Method `CreateSchemaBuilder::build`
    ///
    /// Consumes the builder and returns a `Query<CreateSchema>`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the required fields is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build(self) -> QlResult<Query<CreateSchema>> {
        if self.name.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("CreateSchemaBuilder.name")
            });
        }

        Ok(Query {
            query: CreateSchema {
                name: self.name.unwrap(),
                if_not_exist: self.if_not_exist
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CreateSchema,
        CreateSchemaBuilder,
        QueryKind
    };

    static_assertions::assert_impl_all!(CreateSchema: QueryKind, Send, Sync);
    static_assertions::assert_impl_all!(CreateSchemaBuilder: Send, Sync);
}
