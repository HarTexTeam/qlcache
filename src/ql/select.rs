//! # Module `select`
//!
//! This module implements the `SELECT` query of the query language.

pub(crate) struct Select;

impl Select {
    #[must_use]
    pub(crate) fn builder() -> SelectBuilder {
        SelectBuilder {
            table_name: None,
            scope: None
        }
    }
}


pub struct SelectBuilder {
    pub(crate) table_name: Option<String>,
    pub(crate) scope: Option<SelectScope>
}

impl SelectBuilder {
    #[must_use]
    pub fn table_name(mut self, table_name: String) -> Self {
        self.table_name.replace(table_name);
        self
    }
}

pub enum SelectScope {
    Everything,
    Fields(Vec<String>)
}
