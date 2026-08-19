use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Local, DateTime};

#[derive(Serialize, Deserialize)]
pub struct Reminder {
    id: Uuid,
    title: String,
    place: String,
    status: ReminderState,
    time_at: DateTime<Local>,
    created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
enum ReminderState {
    Active,
    Triggered,
}

pub struct ReminderDraft {
    title: String,
    place: String,
    status: ReminderState,
    time_at: DateTime<Local>,
}