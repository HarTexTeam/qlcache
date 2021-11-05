//! # Module `schema`
//!
//! This module implements the `CREATE SCHEMA` query.

use crate::{
    error::{
        QlError,
        QlResult
    },
    ql::{
        Query,
        QueryKind
    },
    QlCache
};

/// # Struct `CreateSchema`
///
/// A `CREATE SCHEMA` query.
#[allow(clippy::module_name_repetitions, dead_code)]
pub struct CreateSchema {
    pub(crate) name: String
}

impl QueryKind for CreateSchema {
    type ResultType = ();

    fn execute(&self, _: &QlCache) -> QlResult<Self::ResultType> {
        todo!()
    }
}

/// # Struct `CreateSchemaBuilder`
///
/// A builder for a `Create`, constructs a `CREATE SCHEMA` query.
pub struct CreateSchemaBuilder {
    pub(crate) name: Option<String>
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
                name: self.name.unwrap()
            }
        })
    }
}
