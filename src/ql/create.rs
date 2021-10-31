//! # Module `constraints`
//!
//! This module implements constraints for the query language.

#[allow(dead_code)]
pub struct Create {
    pub(crate) kind: CreateKind
}

#[derive(Clone, Eq, PartialEq)]
pub enum CreateKind {
    Table
}
