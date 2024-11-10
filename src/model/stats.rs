use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct StatsDto {
    pub corporation_id: i32,
    pub member_count: i32,
    pub date: DateTime<Utc>,
}

impl Default for StatsDto {
    fn default() -> Self {
        StatsDto {
            corporation_id: 0,
            member_count: 0,
            date: Utc::now(),
        }
    }
}
