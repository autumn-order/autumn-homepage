use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatsDto {
    pub member_count: i32,
    pub date: DateTime<Utc>,
}
