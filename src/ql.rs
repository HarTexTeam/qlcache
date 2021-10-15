//! # Module `ql`
//!
//! This module implements the query language.

use crate::{
    error::{
        ParseQueryErrorKind,
        QlError,
        QlResult
    },
    ColumnValue
};


/// # Enumeration `Command`
///
/// An enumeration representing various commands of the language.
#[derive(Clone, PartialEq)]
enum Command {
    Select(Select),
    None
}

/// # Enumeration `Constraint`
///
/// An enumeration representing a constraint for the query.
#[derive(Clone, PartialEq)]
enum Constraint {
    Equality {
        expected: ColumnValue
    },

    Relational {
        larger_than: Option<ColumnValue>,
        smaller_than: Option<ColumnValue>
    },

    Range {
        upper_limit: Option<ColumnValue>,
        lower_limit: Option<ColumnValue>
    },

    NotNull
}

/// # Struct `Select`
///
/// Represents a `SELECT` command.
#[allow(dead_code)]
#[derive(Clone, PartialEq)]
struct Select {
    table: String,
    values: Values,
    constraints: Option<Vec<Constraint>>
}

/// # Enumeration `Values`
///
/// An enumeration representing the values to query.
#[derive(Clone, PartialEq)]
enum Values {
    Everything,
    Columns(Vec<String>)
}

fn parse(query: &str) -> QlResult<Command> {
    let lines = query.lines();
    let mut command: Command = Command::None;

    for line in lines {
        let mut segments = line.split(" ");

        match segments.next() {
            Some("SELECT") => {
                // obtain the column(s) to `SELECT`
                let values = match segments.next() {
                    Some(value) if value != "*" => Values::Columns(value.split(",").collect()),
                    Some("*") => Values::Everything,
                    _ => return Err(QlError::ParseQueryError(
                        ParseQueryErrorKind::SyntaxError {
                            description: String::from(
                                "expected value to select, found `end of query`"
                            )
                        }
                    ))
                };

                // check if the next lexeme is `FROM`
                let next = segments.peekable().peek();
                if next.is_none() || next.unwrap() != &"FROM" {
                    return Err(QlError::ParseQueryError(
                        ParseQueryErrorKind::SyntaxError {
                            description: String::from(
                                "expected `FROM`, found `end of query` or invalid lexeme"
                            )
                        }
                    ));
                }

                // consume the `FROM`
                segments.next();

                let table = if let Some(table) = segments.next() {
                    table
                }
                else {
                    return Err(QlError::ParseQueryError(
                        ParseQueryErrorKind::SyntaxError {
                            description: String::from(
                                "expected table name, found `end of query`"
                            )
                        }
                    ));
                };

                if segments.peekable().peek().is_none() {
                    // no more code to read, means that the query is simply:
                    // SELECT some_values FROM some_table
                    command = Command::Select(Select {
                        table: table.to_string(),
                        values,
                        constraints: None
                    });
                }
                else {
                    // there are still more lexemes to analyze
                    // check if the next lexeme is `WHERE` for constraints
                    let next = segments.peekable().peek();
                    if next.is_none() || next.unwrap() != &"FROM" {
                        return Err(QlError::ParseQueryError(
                            ParseQueryErrorKind::SyntaxError {
                                description: String::from(
                                    "expected `WHERE`, found `end of query` or invalid lexeme"
                                )
                            }
                        ));
                    }

                    // consume the `WHERE`
                    segments.next();
                }
            }
            Some(command) => {
                return Err(QlError::ParseQueryError(
                    ParseQueryErrorKind::SyntaxError {
                        description: format!("invalid command `{command}`")
                    }
                ));
            }
            None => {
                return Err(QlError::ParseQueryError(
                    ParseQueryErrorKind::SyntaxError {
                        description: String::from("expected command, found `end of query`")
                    }
                ));
            }
        }
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::{
        Command,
        Constraint,
        Select,
        Values
    };

    static_assertions::assert_impl_all!(Command: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Constraint: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Select: Clone, PartialEq, Send, Sync);
    static_assertions::assert_impl_all!(Values: Clone, PartialEq, Send, Sync);
}
