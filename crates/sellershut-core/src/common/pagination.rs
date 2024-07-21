tonic::include_proto!("common.pagination");

#[cfg(any(feature = "rpc-server-categories", feature = "rpc-server-users"))]
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use cursor::Index;

/// Pagination cursor
#[derive(Debug)]
#[cfg(any(feature = "rpc-server-categories", feature = "rpc-server-users"))]
pub struct CursorBuilder {
    id: String,
    idx: i32,
}

#[cfg(any(feature = "rpc-server-categories", feature = "rpc-server-users"))]
impl CursorBuilder {
    /// Create cursor
    pub fn new(id: &str, idx: i32) -> Self {
        Self {
            id: id.to_string(),
            idx,
        }
    }
    /// decode a cursor
    pub fn decode(params: &cursor::cursor_value::CursorType) -> Self {
        let cursor = match params {
            cursor::cursor_value::CursorType::After(cursor) => cursor,
            cursor::cursor_value::CursorType::Before(cursor) => cursor,
        };

        let bytes = BASE64_URL_SAFE_NO_PAD.decode(cursor).unwrap();

        let decoded = String::from_utf8(bytes).unwrap();
        println!("{decoded}");

        let mut tokens = decoded.split(':');
        let idx = tokens.next().unwrap();
        let id = tokens.next().unwrap();

        Self {
            id: id.to_string(),
            idx: idx.parse().unwrap(),
        }
    }

    /// get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// get index
    pub fn idx(&self) -> i32 {
        self.idx
    }

    /// encode a cursor
    pub fn encode(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(format!("{}:{}", self.idx, self.id))
    }

    /// Gets pagination direction
    pub fn is_paginating_from_left(pagination: &Cursor) -> bool {
        if let Some(value) = pagination.cursor_value.as_ref() {
            match value.cursor_type.as_ref() {
                Some(val) => match val {
                    cursor::cursor_value::CursorType::After(_) => true,
                    cursor::cursor_value::CursorType::Before(_) => false,
                },
                None => true,
            }
        } else {
            true
        }
    }

    /// Checks if cursor is unavailable
    pub fn is_cursor_unavailable(pagination: &Cursor) -> bool {
        if let Some(value) = pagination.cursor_value.as_ref() {
            match value.cursor_type.as_ref() {
                Some(val) => match val {
                    cursor::cursor_value::CursorType::After(_) => false,
                    cursor::cursor_value::CursorType::Before(_) => false,
                },
                None => true,
            }
        } else {
            true
        }
    }
}

/// Gets maximum query results from pagination data
pub fn query_count(max: i32, pagination: &Index) -> i32 {
    let user_param = match pagination {
        Index::First(value) => value,
        Index::Last(value) => value,
    };
    if *user_param > max {
        max
    } else {
        *user_param
    }
}
