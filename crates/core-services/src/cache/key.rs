use std::fmt::Display;

use redis::ToRedisArgs;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum CacheKey<'a> {
    Categories(CursorParams<'a>),
    CategoriesSubCategory(CursorParams<'a>),
    Category(&'a str),
}

#[derive(Clone, Copy, Debug)]
pub struct CursorParams<'a> {
    pub cursor: Option<&'a str>,
    pub index: Index,
}

impl Display for CursorParams<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cursor={}:index={}",
            match self.cursor {
                Some(cursor) => cursor,
                None => "[NONE]",
            },
            match self.index {
                Index::After(v) => format!("after:{v}"),
                Index::Before(v) => format!("before:{v}"),
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Index {
    After(i32),
    Before(i32),
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CacheKey::Categories(params) => format!("categories:all:{}", params.to_string()),
                CacheKey::CategoriesSubCategory(params) =>
                    format!("categories:subcategories:{}", params.to_string()),
                CacheKey::Category(id) => format!("categories:id={id}"),
            }
        )
    }
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.to_string().as_bytes())
    }
}
