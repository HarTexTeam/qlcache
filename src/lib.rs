//! # qlcache
//!
//! [![`HarTex` Community](https://img.shields.io/discord/886101109331075103?color=%237289DA&label=HarTex%20Community&logo=discord&style=for-the-badge)](https://discord.gg/Xu8453VBAv)
//!
//! [![GitHub Badge](https://img.shields.io/badge/github-qlcache-6f42c1.svg?style=for-the-badge&logo=github)](https://github.com/HarTexTeam/qlcache)
//! [![License](https://img.shields.io/github/license/HarTexTeam/HarTex-rust-discord-bot?style=for-the-badge&logo=pastebin)](https://www.apache.org/licenses/LICENSE-2.0.txt)
//! ![Minimum Supported Rust Version](https://img.shields.io/badge/rust-1.57-93450a.svg?style=for-the-badge&logo=rust)
//!
//! An object-relational in-memory cache, supports queries with an SQL-like query language.

#![deny(clippy::pedantic, missing_docs, warnings)]
#![feature(format_args_capture)]

use dashmap::DashMap;

pub mod error;
pub mod r#impl;
pub mod ql;

/// # Struct `QlCache`
///
/// A concurrently accessible object-relational in-memory cache.
#[allow(dead_code)]
#[derive(Clone)]
pub struct QlCache {
    pub(crate) cache: DashMap<String, CacheSchema>
}

/// # Struct `CacheSchema`
///
/// A schema in the cache.
#[derive(Clone)]
pub struct CacheSchema {
    /// # Struct Field `name`
    ///
    /// The name of the schema.
    pub name: String,

    /// # Struct Field `tables`
    ///
    /// The tables of the schema.
    pub tables: DashMap<String, CacheTable>
}

/// # Struct `CacheTable`
///
/// A table in the cache.
#[derive(Clone)]
pub struct CacheTable {
    /// # Struct Field `name`
    ///
    /// The name of the table.
    pub name: String,

    /// # Struct Field `columns`
    ///
    /// The columns of the table, and whether the value can be `null`.
    pub columns: DashMap<String, (ColumnDataType, bool)>,

    /// # Struct Field `rows`
    ///
    /// The rows of the table.
    pub rows: DashMap<u64, CacheTableRow>
}

/// # Struct `CacheTableRow`
///
/// A row in a table in the cache.
#[derive(Clone)]
pub struct CacheTableRow {
    /// # Struct Field `column_values`
    ///
    /// The values of each column in this table.
    pub column_values: DashMap<String, ColumnValue>
}

/// # Enumeration `ColumnDataType`
///
/// The datatype of a column. Almost all of the variants correspond to their Rust types.
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Clone, Eq, PartialEq)]
pub enum ColumnDataType {
    // integer types
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,

    // text
    String
}

/// # Enumeration `ColumnValue`
///
/// The value of a column.
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Clone, Eq, PartialEq)]
pub enum ColumnValue {
    // integer types
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    // text
    String(String),

    // null
    Null
}

#[cfg(test)]
mod tests {
    use super::{
        CacheTable,
        CacheTableRow,
        ColumnDataType,
        ColumnValue,
        QlCache
    };

    static_assertions::assert_impl_all!(QlCache: Clone, Send, Sync);
    static_assertions::assert_impl_all!(CacheTable: Clone, Send, Sync);
    static_assertions::assert_impl_all!(CacheTableRow: Clone, Send, Sync);
    static_assertions::assert_impl_all!(ColumnDataType: Clone, Eq, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(ColumnValue: Clone, Eq, PartialEq, Send, Sync);
}
