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
    ColumnDataType,
    QlCache
};

/// # Struct `CreateTable`
///
/// A `CREATE TABLE` query.
#[allow(clippy::module_name_repetitions, dead_code)]
pub struct CreateTable {
    pub(crate) name: String,
    pub(crate) columns: Vec<(String, (ColumnDataType, bool))>,
    pub(crate) primary_key: Option<PrimaryKey>,
    pub(crate) schema: String,
    pub(crate) if_not_exist: bool
}

impl QueryKind for CreateTable {
    type ResultType = ();

    fn execute(&self, _: QlCache) -> QlResult<Self::ResultType> {
        todo!()
    }
}

/// # Struct `CreateTableBuilder`
///
/// A builder for a `Create`, constructs a `CREATE TABLE` query.
///
/// ## Examples
///
/// - `CREATE TABLE TableName`:
/// ```
/// use qlcache::ql::QueryBuilder;
///
/// let create_table = QueryBuilder::create()
///     .table()
///     .name(String::from("TableName"))
///     .build()
///     .unwrap();
/// ```
///
/// - `CREATE TABLE IF NOT EXIST TableName`
/// ```
/// use qlcache::ql::QueryBuilder;;
///
/// let create_table = QueryBuilder::create()
///     .table()
///     .if_not_exist()
///     .name(String::from("TableName"))
///     .build()
///     .unwrap();
/// ```
///
/// - `CREATE TABLE TableName COLUMNS (Field1 STRING NOT NULL, Field2 U64)`
/// ```
/// use qlcache::{
///     ql::QueryBuilder,
///     ColumnDataType
/// };
///
/// let create_table = QueryBuilder::create()
///     .table()
///     .name(String::from("TableName"))
///     .columns(vec![
///         (String::from("Field1"), (ColumnDataType::String, false)),
///         (String::from("Field2"), (ColumnDataType::U64, true))
///     ])
///     .build()
///     .unwrap();
/// ```
///
/// - `CREATE TABLE TableName COLUMNS (Field1 STRING NOT NULL, Field2 U64 PRIMARY KEY)`
/// ```
/// use qlcache::{
///     ql::{
///         key::PrimaryKey,
///         QueryBuilder
///     },
///     ColumnDataType
/// };
///
/// let create_table = QueryBuilder::create()
///     .table()
///     .name(String::from("TableName"))
///     .columns(vec![
///         (String::from("Field1"), (ColumnDataType::String, false)),
///         (String::from("Field2"), (ColumnDataType::U64, true))
///     ])
///     .primary_key(PrimaryKey::new(String::from("Field2")))
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
///
/// - `CREATE TABLE SchemaName.TableName COLUMNS (Field1 STRING NOT NULL, Field2 U64 PRIMARY KEY)`
/// ```
/// use qlcache::{
///     ql::{
///         key::PrimaryKey,
///         QueryBuilder
///     },
///     ColumnDataType
/// };
///
/// let create_table = QueryBuilder::create()
///     .table()
///     .schema(String::from("SchemaName"))
///     .name(String::from("TableName"))
///     .columns(vec![
///         (String::from("Field1"), (ColumnDataType::String, false)),
///         (String::from("Field2"), (ColumnDataType::U64, true))
///     ])
///     .primary_key(PrimaryKey::new(String::from("Field2")))
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
pub struct CreateTableBuilder {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<(String, (ColumnDataType, bool))>,
    pub(crate) primary_key: Option<PrimaryKey>,
    pub(crate) schema: Option<String>,
    pub(crate) if_not_exist: bool
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
        fields: impl IntoIterator<Item = (String, (ColumnDataType, bool))>
    ) -> Self {
        self.columns = fields.into_iter().collect();
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
    #[allow(clippy::missing_panics_doc, clippy::search_is_some)] // this function never panics
    pub fn primary_key(mut self, primary_key: PrimaryKey) -> QlResult<Self> {
        if self.primary_key.is_some() {
            return Err(QlError::PrimaryKeyAlreadySet);
        }

        let iterator = self.columns.iter();

        if let Some((i, field)) = iterator
            .enumerate()
            .find(|(_, (name, _))| name.eq(&primary_key.0))
        {
            self.columns[i] = {
                // primary keys cannot be null, so reflect that
                let mut field = field.clone();
                field.1.1 = false;

                field
            };
        }
        else {
            return Err(QlError::ColumnDoesNotExist);
        }

        self.primary_key.replace(primary_key);
        Ok(self)
    }

    /// # Instance Method `CreateTableBuilder::schema`
    ///
    /// Sets the parent schema for this table to be created.
    ///
    /// ## Parameters
    /// - `schema`, type `String`; the parent schema name
    #[must_use]
    pub fn schema(mut self, schema: String) -> Self {
        self.schema.replace(schema);
        self
    }

    /// # Instance Method `CreateTableBuilder::if_not_exist`
    ///
    /// Sets the table to be created if it does not exist yet, otherwise does nothing.
    #[must_use]
    pub fn if_not_exist(mut self) -> Self {
        self.if_not_exist = true;
        self
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
                primary_key: self.primary_key,
                schema: self.schema.unwrap_or_else(|| String::from("PUBLIC"))
                if_not_exist: self.if_not_exist
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
