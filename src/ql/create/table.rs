//! # Module `table`
//!
//! This module implements the `CREATE TABLE` query.

use crate::{
    error::{
        QlError,
        QlResult
    },
    ql::{
        key::PrimaryKey,
        Query,
        QueryKind
    },
    ColumnDataType
};

/// # Struct `CreateTable`
///
/// A `CREATE TABLE` query.
#[allow(clippy::module_name_repetitions, dead_code)]
pub struct CreateTable {
    pub(crate) name: String,
    pub(crate) columns: Vec<(String, (ColumnDataType, bool))>,
    pub(crate) primary_key: Option<PrimaryKey>
}

impl QueryKind for CreateTable {}

/// # Struct `CreateTableBuilder`
///
/// A builder for a `Create`, constructs a `CREATE TABLE` query.
pub struct CreateTableBuilder {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<(String, (ColumnDataType, bool))>,
    pub(crate) primary_key: Option<PrimaryKey>
}

impl CreateTableBuilder {
    /// # Instance Method `CreateTableBuilder::name`
    ///
    /// Sets the name of the object to create.
    ///
    /// ## Parameters
    /// - `name`, type `String`; the table name
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name.replace(name);
        self
    }

    /// # Instance Method `CreateTableBuilder::name`
    ///
    /// Sets the name of the object to create.
    ///
    /// ## Parameters
    /// - `fields`, type `impl Iterator<Item = (String, (ColumnDataType, bool))`; an iterator over
    ///             the fields, where:
    ///               - `TUPLE.0`, type `String`; the name of the column
    ///               - `TUPLE.1.0`, type `ColumnDataType`; the data type of the column
    ///               - `TUPLE.1.1`, type `bool`; whether the field value can be `NULL`.
    #[must_use]
    pub fn columns(
        mut self,
        fields: impl Iterator<Item = (String, (ColumnDataType, bool))>
    ) -> Self {
        self.columns = fields.collect();
        self
    }

    /// # Instance Method `CreateTableBuilder::primary_key`
    ///
    /// Sets the primary key for this table.
    ///
    /// ## Parameters
    /// - `primary_key`, type `PrimaryKey`; the primary key
    ///
    /// ## Errors
    ///
    /// Returns `PrimaryKeyAlreadySet` if the primary key of the table has already been set;
    /// returns `ColumnDoesNotExist` of the name of the primary key does not exist as a column in
    /// the table.
    pub fn primary_key(mut self, primary_key: PrimaryKey) -> QlResult<Self> {
        if self.primary_key.is_some() {
            return Err(QlError::PrimaryKeyAlreadySet);
        }

        let mut iterator = self.columns.iter();

        if iterator.find(|(name, _)| *name.eq(&primary_key.0)).is_none() {
            return Err(QlError::ColumnDoesNotExist);
        }

        let (i, &mut mut field) = iterator
            .enumerate()
            .find(|(_, (name, _))| *name.eq(&primary_key.0))
            .unwrap();

        field.1.1 = false;
        self.columns[i] = field;

        self.primary_key.replace(primary_key);
        Ok(self)
    }

    /// # Instance Method `CreateTableBuilder::build`
    ///
    /// Consumes the builder and returns a `Query<CreateTable>`.
    ///
    /// ## Errors
    ///
    /// Returns `RequiredFieldIsNone` if any of the fields required is `None`.
    #[allow(clippy::missing_panics_doc)] // this function never panics
    pub fn build(self) -> QlResult<Query<CreateTable>> {
        if self.name.is_none() {
            return Err(QlError::RequiredFieldIsNone {
                field_name: String::from("name")
            });
        }

        Ok(Query {
            query: CreateTable {
                name: self.name.unwrap(),
                columns: self.columns,
                primary_key: self.primary_key
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CreateTable,
        CreateTableBuilder,
        QueryKind
    };

    static_assertions::assert_impl_all!(CreateTable: QueryKind, Send, Sync);
    static_assertions::assert_impl_all!(CreateTableBuilder: Send, Sync);
}
