use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

struct Date(DateTime<Utc>);

#[derive(Default, Debug, Serialize, Deserialize)]
struct Item {
    id: ItemId,                  // i64
    item_kind: Option<ItemKind>, // Task | Note | UserStory | BacklogItem | Issue | Milestone
    size: Option<Size>,          // Hour(i32) | StoryPoint(i32)
    remaining: Option<Size>,     // same as above
    hour_spent: f32,             // Hours spent on this item; calculated by sum of log
    log: Vec<LogItem>,           // Log item
    title: Option<String>,       // Optional
    description: Option<String>, // Optional
    priority: Option<Priority>,  // 1 | 2 | 3
    owner: Option<UserId>,       //
    duedate: Option<NaiveDate>,  //
    created_at: DateTime<Utc>,   //
    created_by: UserId,          //
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct LogItem {
    id: String,
    hours_spent: f32,
    remaining_size: Size,
    log_message: String,
    created_at: DateTime<Utc>,
    created_by: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
enum ItemKind {
    Task,
    Note,
    UserStory,
    BacklogItem,
    Issue,
    Milestone,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ItemId(i64);

#[derive(Debug, Serialize, Deserialize, Default)]
struct SprintId(i64);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Size {
    Unknown,
    Hour(i32),
    StoryPoint(i32),
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Size::Unknown => panic!("Unknown command should not be printed!"),
            Size::Hour(h) => format!("{}h", h),
            Size::StoryPoint(p) => format!("{}p", p),
        }
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Cut input str as char vector
        let mut s_vec: Vec<char> = s.trim().chars().collect();
        // Get last character as unit character
        let unit_char = match s_vec.pop() {
            Some(lc) => lc,
            None => return Err("Size should not be an empty string".to_string()),
        };
        // Check if number can be parsed
        let number: i32 = match s_vec.iter().collect::<String>().parse::<i32>() {
            Ok(res) => res,
            Err(_) => return Err("Number cannot be parsed".to_string()),
        };
        // Check if type correct
        match &unit_char {
            'h' => Ok(Size::Hour(number)),
            'p' => Ok(Size::StoryPoint(number)),
            _ => return Err("Wrong size unit. h or p".to_string()),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    I,
    II,
    III,
}

impl Default for Priority {
    fn default() -> Self {
        Self::III
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct UserId(String);

struct Project {
    id: String,
    items: Vec<Item>,
    owner: UserId,
}

impl Project {
    fn add_entry(&mut self, entry: Entry) -> Result<(), String> {
        todo!()
    }
}

enum ItemParameter {
    Title(String),
    Description(String),
    Size(Size),
    Remaining(Size),
    Spent(Size),
    Priority(Priority),
    Owner(UserId),
    Duedate(Date),
    Kind(ItemKind),
}

impl ToString for ItemParameter {
    fn to_string(&self) -> String {
        match self {
            ItemParameter::Title(t) => format!("title {}", t.to_owned()),
            ItemParameter::Description(t) => format!("description {}", t.to_string()),
            ItemParameter::Size(s) => todo!(),
            ItemParameter::Remaining(_) => todo!(),
            ItemParameter::Spent(_) => todo!(),
            ItemParameter::Priority(_) => todo!(),
            ItemParameter::Owner(_) => todo!(),
            ItemParameter::Duedate(_) => todo!(),
            ItemParameter::Kind(_) => todo!(),
        }
    }
}

enum LogParameter {
    Spent(Size),
    Remaining(Size),
    Message(String),
}

enum Command {
    Create {
        item_id: Uuid,
    },
    Set {
        item_id: Uuid,
        parameters: Vec<ItemParameter>,
    },
    Log {
        item_id: Uuid,
        parameters: Vec<LogParameter>,
    },
}

struct Entry {
    id: Uuid,
    datetime: DateTime<Utc>,
    userid: UserId,
    command: Command,
}

struct ProjectLog {
    log_file_path: PathBuf,
    log_entries: Vec<Entry>,
}

trait ProjectLogExt {
    fn from_log_file(log_file: PathBuf) -> Result<ProjectLog, String>;
    fn write_to_file(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_size() {
        assert_eq!(Size::from_str("3h").unwrap(), Size::Hour(3));
        assert_eq!(Size::from_str("3p").unwrap(), Size::StoryPoint(3));
        assert_eq!(Size::from_str("3m").is_err(), true);
        assert_eq!(Size::from_str("h").is_err(), true);
        assert_eq!(Size::from_str("3").is_err(), true);
        assert_eq!(Size::from_str("").is_err(), true);
        assert_eq!(Size::from_str("3hp").is_err(), true);
    }
}
