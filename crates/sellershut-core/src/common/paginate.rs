use crate::common::Paginate;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};

/// Pagination cursor
#[derive(Debug)]
pub struct Cursor {
    id: String,
    idx: i32,
}

impl Cursor {
    /// Create cursor
    pub fn new(id: &str, idx: i32) -> Self {
        Self {
            id: id.to_string(),
            idx,
        }
    }
    /// decode a cursor
    pub fn decode(params: Paginate) -> Self {
        let _count = params.first.unwrap_or_else(|| params.last());

        let cursor = params.after.map_or_else(|| params.before, Some).unwrap();

        let bytes = BASE64_URL_SAFE_NO_PAD.decode(&cursor).unwrap();

        let decoded = String::from_utf8(bytes).unwrap();
        println!("decoding: {decoded}");

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
}
