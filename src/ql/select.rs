//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

pub(crate) struct Select;

pub struct SelectBuilder {
    pub(crate) table_name: Option<String>,
    pub(crate) scope: Option<SelectScope>
}

pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}
