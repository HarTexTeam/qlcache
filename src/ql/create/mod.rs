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
    /// Sets the kind of object to create in the cache.
    ///
    /// ## Parameters
    /// - `kind`, type `CreateKind`; the object kind
    #[must_use]
    pub fn table(mut self) -> CreateTableBuilder {
        CreateTableBuilder {
            name: None
        }
    }
}
