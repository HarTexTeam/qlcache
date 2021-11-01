//! # Module `create`
//!
//! This module implements the `CREATE` query.

use crate::ql::create::table::CreateTableBuilder;

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
    /// # Instance Method `CreateBuilder::kind`
    ///
    /// Returns a builder for a `CREATE TABLE` query.
    #[must_use]
    pub fn table(self) -> CreateTableBuilder {
        CreateTableBuilder {
            name: None
        }
    }
}
