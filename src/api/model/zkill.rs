use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ZkillStatsInfo {
    pub member_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct ZkillStats {
    pub info: ZkillStatsInfo,
    pub ships_destroyed: i32,
}
