use crate::common::Paginate;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};

/// Pagination cursor
#[derive(Debug)]
pub struct Cursor {
    id: String,
    created_at: i64,
}

impl Cursor {
    /// Create cursor
    pub fn new(id: &str, created_at: i64) -> Self {
        Self {
            id: id.to_string(),
            created_at,
        }
    }
    /// decode a cursor
    pub fn decode(params: Paginate) -> Self {
        let _count = params.first.unwrap_or_else(|| params.last());

        let cursor = params.after.map_or_else(|| params.before, Some).unwrap();

        let bytes = BASE64_URL_SAFE_NO_PAD.decode(&cursor).unwrap();

        let decoded = String::from_utf8(bytes).unwrap();

        let mut tokens = decoded.split(':');
        let first = tokens.next().unwrap();
        let last = tokens.next().unwrap();

        Self {
            id: first.to_string(),
            created_at: last.parse().unwrap(),
        }
    }

    /// get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// get created_at
    pub fn created_at(&self) -> i64 {
        self.created_at
    }

    /// encode a cursor
    pub fn encode(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(format!("{}:{}", self.created_at, self.id))
    }
}
