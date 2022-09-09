use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    command::UserInput,
    item::{Date, ItemKind, ItemParameter, LogParameter, Priority, Size, UserId},
};

fn uuid_from_str(s: &str) -> Result<Uuid, String> {
    Uuid::from_str(s).map_err(|_| "Wrong item ID format. Must be UUID".to_string())
}

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

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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

impl FromStr for EntryKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<&str>>();
        let cmd_str = v.get(0).ok_or("No cmd found".to_string())?;
        let id = v.get(1).ok_or("No id found".to_string())?;
        let _params = v[2..].join(" ");
        let _params = _params.split(";").collect::<Vec<&str>>();
        let mut params = Vec::new();
        for p in _params {
            if p.len() > 0 {
                params.push(Parameter::from_str(p)?);
            }
        }
        match *cmd_str {
            "create" | "CREATE" => Ok(Self::Create {
                id: uuid_from_str(id)?,
            }),
            "set" | "SET" => match *id {
                "project" => Ok(Self::Set {
                    kind: SetKind::Project,
                    params,
                }),
                _ => Ok(Self::Set {
                    kind: SetKind::Item(uuid_from_str(id)?),
                    params,
                }),
            },
            "log" | "LOG" => Ok(Self::Log {
                id: uuid_from_str(id)?,
                params,
            }),
            _ => Err("Unkown entrykind".to_string()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LogEntry {
    id: Uuid,
    userid: String,
    date: Date,
    entry_kind: EntryKind,
}

impl LogEntry {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn userid(&self) -> &str {
        &self.userid
    }
    pub fn date(&self) -> &Date {
        &self.date
    }
    pub fn entry_kind(&self) -> &EntryKind {
        &self.entry_kind
    }
    pub fn from_user_input(i: &UserInput, cmd_str: &str) -> Result<Self, String> {
        let entry_kind = EntryKind::from_str(cmd_str)?;
        Ok(Self {
            id: i.id().to_owned(),
            userid: i.userid().to_string(),
            date: i.date().to_owned(),
            entry_kind,
        })
    }
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.id.as_simple(),
            self.date,
            &self.userid,
            self.entry_kind
        )
    }
}

impl FromStr for LogEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<&str>>();
        let id = uuid_from_str(v.get(0).ok_or("No ID found".to_string())?)?;
        let date = Date::from_str(v.get(1).ok_or("No date found".to_string())?)?;
        let userid = v.get(2).ok_or("No userid found".to_string())?.to_string();
        let entry_kind = EntryKind::from_str(&v[3..].join(" "))?;
        Ok(Self {
            id,
            userid,
            date,
            entry_kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::item::{Priority, Size};

    use super::*;

    #[test]
    fn entry_to_string() {
        let id = Uuid::new_v4();
        let date = Date::now();
        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date,
            entry_kind: EntryKind::Create { id },
        };
        let t = format!(
            "{} {} {} CREATE {}",
            id.as_simple(),
            date,
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
            date,
            "mezeipetister",
            id.as_simple()
        );
        assert_eq!(t, entry.to_string());

        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date: date,
            entry_kind: EntryKind::Set {
                kind: SetKind::Project,
                params: vec![
                    Parameter::Title("Hello bello".to_string()),
                    Parameter::Priority(Priority::I),
                ],
            },
        };
        let t = format!(
            "{} {} {} SET {} title Hello bello;priority 1",
            id.as_simple(),
            date,
            "mezeipetister",
            "project"
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
            date,
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

    #[test]
    fn log_entry_write_parse() {
        let id = Uuid::new_v4();
        let date = Date::now();
        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date,
            entry_kind: EntryKind::Create { id },
        };
        let result = LogEntry::from_str(&entry.to_string()).unwrap();
        assert_eq!(entry, result);

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
        let result = LogEntry::from_str(&entry.to_string()).unwrap();
        assert_eq!(entry, result);

        let entry = LogEntry {
            id,
            userid: "mezeipetister".to_string(),
            date: date,
            entry_kind: EntryKind::Set {
                kind: SetKind::Project,
                params: vec![Parameter::Title("Hello bello".to_string())],
            },
        };
        let result = LogEntry::from_str(&entry.to_string()).unwrap();
        assert_eq!(entry, result);

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
        let result = LogEntry::from_str(&entry.to_string()).unwrap();
        assert_eq!(entry, result);
    }
}
