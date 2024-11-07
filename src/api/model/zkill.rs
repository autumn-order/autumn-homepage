use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ZkillStatsInfo {
    pub member_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct ZkillStats {
    pub info: ZkillStatsInfo,
    pub ships_destroyed: u32,
}
