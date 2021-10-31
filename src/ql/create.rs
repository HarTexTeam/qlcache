//! # Module `constraints`
//!
//! This module implements constraints for the query language.

/// # Struct `Create`
///
/// A `CREATE` query.
#[allow(dead_code)]
pub struct Create {
    pub(crate) kind: CreateKind
}

impl Create {
    pub(crate) fn builder() -> CreateBuilder {
        CreateBuilder {
            kind: None
        }
    }
}

/// # Struct `CreateBuilder`
///
/// A builder for a `Create`, constructs a `CREATE` query.
#[allow(clippy::module_name_repetitions)]
pub struct CreateBuilder {
    pub(crate) kind: Option<CreateKind>,
    pub(crate) object_name: Option<String>
}

impl CreateBuilder {
    /// # Instance Method `CreateBuilder::kind`
    ///
    /// Sets the kind of object to create in the cache.
    ///
    /// ## Parameters
    /// - `kind`, type `CreateKind`; the object kind
    #[must_use]
    pub fn kind(mut self, kind: CreateKind) -> Self {
        self.kind.replace(kind);
        self
    }

    /// # Instance Method `CreateBuilder::object_name`
    ///
    /// Sets the name of the object to create.
    ///
    /// ## Parameters
    /// - `kind`, type `String`; the object name
    #[must_use]
    pub fn object_name(mut self, object_name: String) -> Self {
        self.object_name.replace(object_name);
        self
    }
}

/// # Enumeration `CreateKind`
///
/// The kind of object to create.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Eq, PartialEq)]
pub enum CreateKind {
    Table
}
