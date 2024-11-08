use chrono::{DateTime, Utc};

pub struct StatsDto {
    pub member_count: i32,
    pub date: DateTime<Utc>,
}
