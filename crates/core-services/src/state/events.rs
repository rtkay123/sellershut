use std::{fmt::Display, str::FromStr};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
/// Entity type
pub enum Entity {
    /// Categories
    Categories,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Entity::Categories => "categories",
            }
        )
    }
}

impl FromStr for Entity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "categories" => Ok(Self::Categories),
            _ => Err(()),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
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

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('.');
        let entity = tokens.next().ok_or(())?;

        let entity = Entity::from_str(entity)?;

        // Extract and match the action part
        match tokens.next() {
            Some("update") => match tokens.next() {
                Some("index") => match tokens.next() {
                    Some("set") => Ok(Event::SetAll(entity)),
                    Some("update") => Ok(Event::UpdateAll(entity)),
                    Some("delete") => Ok(Event::DeleteAll(entity)),
                    _ => Err(()),
                },
                Some("set") => Ok(Event::UpdateCache(entity)),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}
