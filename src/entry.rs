use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::item::{ItemParameter, LogParameter};

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
        params: Vec<ItemParameter>,
    },
    Log {
        id: Uuid,
        params: Vec<LogParameter>,
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

impl ToString for LogEntry {
    fn to_string(&self) -> String {
        format!(
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
                    ItemParameter::Title("Hello bello".to_string()),
                    ItemParameter::Priority(Priority::I),
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
                    LogParameter::Spent(Size::Hour(2)),
                    LogParameter::Remaining(Size::StoryPoint(4)),
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
}
