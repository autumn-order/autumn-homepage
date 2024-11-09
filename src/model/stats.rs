use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StatsDto {
    pub corporation_ids: i32,
    pub member_count: i32,
    pub date: DateTime<Utc>,
}
