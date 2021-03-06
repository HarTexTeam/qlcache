//! # Module `create`
//!
//! This module implements the `CREATE` query.

use crate::ql::create::{
    schema::CreateSchemaBuilder,
    table::CreateTableBuilder
};

pub mod schema;
pub mod table;

/// # Struct `Create`
///
/// A `CREATE` query.
#[allow(dead_code)]
pub struct Create;

impl Create {
    pub(crate) fn builder() -> CreateBuilder {
        CreateBuilder
    }
}

/// # Struct `CreateBuilder`
///
/// A builder for a `Create`, constructs a `CREATE` query.
#[allow(clippy::module_name_repetitions)]
pub struct CreateBuilder;

impl CreateBuilder {
    /// # Instance Method `CreateBuilder::schema`
    ///
    /// Returns a builder for a `CREATE SCHEMA` query.
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn schema(self) -> CreateSchemaBuilder {
        CreateSchemaBuilder {
            name: None,
            if_not_exist: false
        }
    }

    /// # Instance Method `CreateBuilder::table`
    ///
    /// Returns a builder for a `CREATE TABLE` query.
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn table(self) -> CreateTableBuilder {
        CreateTableBuilder {
            name: None,
            columns: Vec::new(),
            primary_key: None,
            schema: None,
            if_not_exist: false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Create,
        CreateBuilder
    };

    static_assertions::assert_impl_all!(Create: Send, Sync);
    static_assertions::assert_impl_all!(CreateBuilder: Send, Sync);
}
