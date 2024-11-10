use entity::stats::Model;

use crate::model::stats::StatsDto;

impl From<Model> for StatsDto {
    fn from(model: Model) -> Self {
        StatsDto {
            corporation_id: model.corporation_id,
            member_count: model.member_count,
            date: model.date,
        }
    }
}
