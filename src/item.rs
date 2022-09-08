use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

struct Date(DateTime<Utc>);

impl ToString for Date {
    fn to_string(&self) -> String {
        self.0.to_rfc3339()
    }
}

impl Date {
    fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Item {
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

impl ToString for ItemKind {
    fn to_string(&self) -> String {
        let value = match self {
            ItemKind::Task => "task",
            ItemKind::Note => "note",
            ItemKind::UserStory => "user_story",
            ItemKind::BacklogItem => "backlog_item",
            ItemKind::Issue => "issue",
            ItemKind::Milestone => "milestone",
        };
        format!("kind {}", value)
    }
}

impl FromStr for ItemKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "task" => Ok(Self::Task),
            "note" => Ok(Self::Note),
            "user_story" => Ok(Self::UserStory),
            "backlog_item" => Ok(Self::BacklogItem),
            "issue" => Ok(Self::Issue),
            "milestone" => Ok(Self::Milestone),
            _ => Err("Unknown kind format".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ItemId(i64);

#[derive(Debug, Serialize, Deserialize, Default)]
struct SprintId(i64);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Size {
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
pub enum Priority {
    I,
    II,
    III,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        let value = match self {
            Priority::I => 1,
            Priority::II => 2,
            Priority::III => 3,
        };
        format!("{}", value)
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Self::I),
            "2" => Ok(Self::II),
            "3" => Ok(Self::III),
            _ => Err("Wring priority format, 1|2|3".to_string()),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::III
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct UserId(String);

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

struct Project {
    id: String,
    items: Vec<Item>,
    owner: UserId,
}

impl Project {
    fn add_entry(&mut self, entry: EntryCommand) -> Result<(), String> {
        todo!()
    }
}

pub enum ItemParameter {
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
            ItemParameter::Size(s) => format!("size {}", s.to_string()),
            ItemParameter::Remaining(r) => format!("remaining {}", r.to_string()),
            ItemParameter::Spent(s) => format!("spent {}", s.to_string()),
            ItemParameter::Priority(p) => format!("priority {}", p.to_string()),
            ItemParameter::Owner(o) => format!("owner {}", o.to_string()),
            ItemParameter::Duedate(d) => format!("duedate {}", d.to_string()),
            ItemParameter::Kind(k) => format!("kind {}", k.to_string()),
        }
    }
}

impl FromStr for ItemParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param_key = s
            .split_whitespace()
            .collect::<Vec<&str>>()
            .first()
            .ok_or("")?
            .to_string();

        if param_key.len() == 0 {
            return Err("No param key found!".to_string());
        }

        let param_str = s.split_whitespace().collect::<Vec<&str>>()[1..].join(" ");

        match param_key.as_str() {
            "title" => Ok(Self::Title(param_str)),
            "description" => Ok(Self::Description(param_str)),
            "size" => todo!(),
            "remaining" => todo!(),
            "spent" => todo!(),
            "priority" => todo!(),
            "owner" => Ok(Self::Owner(UserId(param_str))),
            "duedate" => todo!(),
            "kind" => todo!(),
            x => Err(format!("Unknown item parameter: {}", x)),
        }
    }
}

pub enum LogParameter {
    Spent(Size),
    Remaining(Size),
    Message(String),
}

impl ToString for LogParameter {
    fn to_string(&self) -> String {
        match self {
            LogParameter::Spent(s) => format!("spent {}", s.to_string()),
            LogParameter::Remaining(r) => format!("remaining {}", r.to_string()),
            LogParameter::Message(m) => format!("message {}", m.to_string()),
        }
    }
}

fn params_to_string<T: ToString>(params: &Vec<T>) -> String {
    let mut res = String::new();
    let mut first = false;
    params.iter().for_each(|p| {
        if !first {
            res.push_str("; ");
        }
        res.push_str(&p.to_string());
    });
    res
}

enum EntryCommand {
    Create {
        item_id: Uuid,
    },
    Set {
        item_id: Uuid,
        params: Vec<ItemParameter>,
    },
    Log {
        item_id: Uuid,
        params: Vec<LogParameter>,
    },
}

impl FromStr for EntryCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let cmd = match parts.first() {
            Some(f) => f.trim().to_owned(),
            None => return Err("Command must contains at least one word".to_string()),
        };
        let item_id = match parts.get(1) {
            Some(idstr) => Uuid::from_str(idstr.trim())
                .map_err(|_| "Wrong item ID format. Must be UUID".to_string())?,
            None => return Err("No new task item id provided".to_string()),
        };
        // let params = parts[2..]
        //     .join(" ")
        //     .split(";")
        //     .collect::<Vec<&str>>()
        //     .iter()
        //     .map(|p| p.split_whitespace())
        //     .collect::<Vec<(&str, &str)>>();
        let res = match cmd.as_str() {
            "create" | "CREATE" => Self::Create { item_id },
            "set" | "SET" => Self::Set {
                item_id,
                params: Vec::new(),
            },
            "log" | "LOG" => Self::Log {
                item_id,
                params: Vec::new(),
            },
            _ => return Err("Unknown entry command".to_string()),
        };
        Ok(res)
    }
}

impl EntryCommand {
    fn create_parse_str(param_str: &str) -> Result<Self, String> {
        todo!()
    }
    fn set_parse_str(param_str: &str) -> Result<Self, String> {
        todo!()
    }
    fn log_parse_str(param_str: &str) -> Result<Self, String> {
        todo!()
    }
}

impl ToString for EntryCommand {
    fn to_string(&self) -> String {
        match self {
            EntryCommand::Create { item_id } => format!("CREATE {}", item_id.as_hyphenated()),
            EntryCommand::Set { item_id, params } => format!(
                "SET {} {}",
                item_id.as_hyphenated(),
                params_to_string(params)
            ),
            EntryCommand::Log { item_id, params } => format!(
                "LOG {} {}",
                item_id.as_hyphenated(),
                params_to_string(params)
            ),
        }
    }
}

struct ProjectLog {
    log_file_path: PathBuf,
    log_entries: Vec<()>,
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

    #[test]
    fn size_to_string() {
        assert_eq!(Size::Hour(3).to_string().as_str(), "3h");
        assert_eq!(Size::Hour(-3).to_string().as_str(), "-3h");
        assert_eq!(Size::Hour(20).to_string().as_str(), "20h");
        assert_eq!(Size::StoryPoint(2).to_string().as_str(), "2p");
        assert_eq!(Size::StoryPoint(-2).to_string().as_str(), "-2p");
    }

    #[test]
    fn command_to_string() {
        let command = EntryCommand::Create {
            item_id: Uuid::new_v4(),
        };
        assert_eq!(command.to_string().len() > 0, true);
    }
}
