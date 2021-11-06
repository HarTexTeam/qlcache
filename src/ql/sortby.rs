//! # Module `sortby`
//!
//! This module implements `SORT BY`, where items are sorted with the columns provided, as well as
//! the order to sort for, whether it is ascending or descending order.

use crate::error::{
    QlError,
    QlResult
};

/// # Struct `SortBy`
///
/// Sort items according to a certain column and a certain order.
#[allow(dead_code)]
pub struct SortBy {
    pub(crate) columns: Vec<String>,
    pub(crate) order: SortOrdering
}

impl SortBy {
    /// # Constructor `SortBy::new`
    ///
    /// Constructs a new `SortBy`.
    ///
    /// ## Errors
    ///
    /// Returns `VecCannotBeEmpty` if the `Vec` of the `columns` parameter is empty.
    pub fn new(columns: Vec<String>, order: Option<SortOrdering>) -> QlResult<Self> {
        if columns.is_empty() {
            return Err(QlError::VecCannotBeEmpty {
                vec_name: String::from("Sortby.columns")
            });
        }

        Ok(Self {
            columns,
            order: order.unwrap_or(SortOrdering::Ascending)
        })
    }
}

/// # Struct `SortOrdering`
///
/// Sort items according to a certain column and a certain order.
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Clone, Eq, PartialEq)]
pub enum SortOrdering {
    Ascending,
    Descending
}

#[cfg(test)]
mod tests {
    use super::{
        SortBy,
        SortOrdering
    };

    static_assertions::assert_impl_all!(SortBy: Send, Sync);
    static_assertions::assert_impl_all!(SortOrdering: Clone, Eq, PartialEq, Send, Sync);
}
