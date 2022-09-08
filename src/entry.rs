use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::item::{Date, ItemKind, ItemParameter, LogParameter, Priority, Size, UserId};

#[derive(PartialEq, Debug)]
pub enum Parameter {
    Title(String),
    Description(String),
    Size(Size),
    Remaining(Size),
    Spent(Size),
    Priority(Priority),
    Owner(UserId),
    Duedate(Date),
    Kind(ItemKind),
    Message(String),
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (key, value) = match self {
            Parameter::Title(c) => ("title", c.to_string()),
            Parameter::Description(c) => ("description", c.to_string()),
            Parameter::Size(c) => ("size", c.to_string()),
            Parameter::Remaining(c) => ("remaining", c.to_string()),
            Parameter::Spent(c) => ("spent", c.to_string()),
            Parameter::Priority(c) => ("priority", c.to_string()),
            Parameter::Owner(c) => ("owner", c.to_string()),
            Parameter::Duedate(c) => ("duedate", c.to_string()),
            Parameter::Kind(c) => ("kind", c.to_string()),
            Parameter::Message(c) => ("message", c.to_string()),
        };
        write!(f, "{} {}", key, value)
    }
}

impl FromStr for Parameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<&str>>();
        let key = v.get(0).ok_or("No key found for parameter".to_string())?;
        let param = v[1..].join(" ");
        match *key {
            "title" => Ok(Self::Title(param)),
            "description" => Ok(Self::Description(param)),
            "size" => Ok(Self::Size(Size::from_str(&param)?)),
            "remaining" => Ok(Self::Remaining(Size::from_str(&param)?)),
            "spent" => Ok(Self::Spent(Size::from_str(&param)?)),
            "priority" => Ok(Self::Priority(Priority::from_str(&param)?)),
            "owner" => Ok(Self::Owner(UserId(param))),
            "duedate" => Ok(Self::Duedate(Date::from_str(&param)?)),
            "kind" => Ok(Self::Kind(ItemKind::from_str(&param)?)),
            "message" => Ok(Self::Message(param)),
            _ => Err("Unkown parameter".to_string()),
        }
    }
}

pub enum SetKind {
    Project,
    Item(Uuid),
}

impl Display for SetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetKind::Project => write!(f, "project"),
            SetKind::Item(id) => write!(f, "{}", id.as_simple()),
        }
    }
}

pub enum EntryKind {
    Create {
        id: Uuid,
    },
    Set {
        kind: SetKind,
        params: Vec<Parameter>,
    },
    Log {
        id: Uuid,
        params: Vec<Parameter>,
    },
}

impl Display for EntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryKind::Create { id } => write!(f, "CREATE {}", id.as_simple()),
            EntryKind::Set { kind, params } => write!(
                f,
                "SET {} {}",
                kind,
                params
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(";")
            ),
            EntryKind::Log { id, params } => write!(
                f,
                "LOG {} {}",
                id.as_simple(),
                params
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(";")
            ),
        }
    }
}

pub struct LogEntry {
    id: Uuid,
    userid: String,
    date: DateTime<Utc>,
    entry_kind: EntryKind,
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.id.as_simple(),
            self.date.to_rfc3339(),
            &self.userid,
            self.entry_kind
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::item::{Priority, Size};

    use super::*;

    #[test]
    fn entry_to_string() {
        let id = Uuid::new_v4();
        let date = Utc::now();
        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date,
            entry_kind: EntryKind::Create { id },
        };
        let t = format!(
            "{} {} {} CREATE {}",
            id.as_simple(),
            date.to_rfc3339(),
            "mezeipetister",
            id.as_simple()
        );
        assert_eq!(t, entry.to_string());

        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date: date,
            entry_kind: EntryKind::Set {
                kind: SetKind::Item(id),
                params: vec![
                    Parameter::Title("Hello bello".to_string()),
                    Parameter::Priority(Priority::I),
                ],
            },
        };
        let t = format!(
            "{} {} {} SET {} title Hello bello;priority 1",
            id.as_simple(),
            date.to_rfc3339(),
            "mezeipetister",
            id.as_simple()
        );
        assert_eq!(t, entry.to_string());

        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date,
            entry_kind: EntryKind::Log {
                id,
                params: vec![
                    Parameter::Spent(Size::Hour(2)),
                    Parameter::Remaining(Size::StoryPoint(4)),
                ],
            },
        };
        let t = format!(
            "{} {} {} LOG {} spent 2h;remaining 4p",
            id.as_simple(),
            date.to_rfc3339(),
            "mezeipetister",
            id.as_simple()
        );
        assert_eq!(t, entry.to_string());
    }

    #[test]
    fn prama_parse() {
        assert_eq!(
            Parameter::from_str("size 2p").unwrap(),
            Parameter::Size(Size::StoryPoint(2))
        );
        assert_eq!(
            Parameter::from_str("title hello bello dolla").unwrap(),
            Parameter::Title("hello bello dolla".to_string())
        );
        let n = Utc::now();
        assert_eq!(
            Parameter::from_str(&format!("duedate {}", n.to_rfc3339())).unwrap(),
            Parameter::Duedate(Date::new(n))
        );
    }
}
