use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, Clone)]
/// Entity type
pub enum Entity {
    /// Categories
    Categories(std::sync::Arc<str>),
}

impl Entity {
    pub fn categories(name: String) -> Self {
        Entity::Categories(name.into())
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Entity::Categories(name) => name,
            }
        )
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
/// Events
pub enum Event {
    /// Sets cache and search index
    SetAll(Entity),
    /// Updates cache and search index
    UpdateAll(Entity),
    /// Deletes cache and search index
    DeleteAll(Entity),
    /// Updates cache only
    UpdateCache(Entity),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Event::SetAll(entity) => {
                    format!("{entity}.update.index.set")
                }
                Event::UpdateAll(entity) => {
                    format!("{entity}.update.index.update")
                }
                Event::DeleteAll(entity) => {
                    format!("{entity}.update.index.delete")
                }
                Event::UpdateCache(entity) => {
                    format!("{entity}.update.set")
                }
            }
        )
    }
}
