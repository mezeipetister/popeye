struct Item {
    id: ItemId,                  // i64
    parent_id: ItemId,           // i64
    sprint_id: SprintId,         // String / Hexadecimal i64 string
    item_kind: Option<ItemKind>, // Task | Note | UserStory | BacklogItem | Issue | Milestone
    size: Option<Size>,          // Hour(i32) | StoryPoint(i32)
    remaining: Option<Size>,     // same as above
    hour_spent: f32,             // Hours spent on this item; calculated by sum of log
    log: Vec<LogItem>,           // Log item
    title: Option<String>,       // Optional
    description: Option<String>, // Optional
    priority: Option<Priority>,  // 1 | 2 | 3
    owner: Option<UserId>,       //
    start_after: Option<ItemId>, // Waterflow or general guard
    deadline: Option<NaiveDate>, //
    created_at: DateTime<Utc>,   //
    created_by: UserId,          //
}

struct LogItem {
    id: Uuid,
    hours_spent: f32,
    remaining_size: Size,
    log_message: String,
    created_at: DateTime<Utc>,
    created_by: UserId,
}

enum ItemKind {
    Task,
    Note,
    UserStory,
    BacklogItem,
    Issue,
    Milestone,
}

struct ItemId(i64);

struct SprintId(i64);

enum Size {
    Hour(i32),
    StoryPoint(i32),
}

enum Priority {
    I,
    II,
    III,
}

struct UserId(String);

struct Workspace {}
