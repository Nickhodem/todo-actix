use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ToDo{
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub create_time: Option<DateTime<Utc>>
}