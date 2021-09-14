use crate::{cost::Cost, cost_entity::CostEntity};

impl From<CostEntity> for Cost {
    fn from(cost_entity: CostEntity) -> Cost {
        Cost {
            id: cost_entity.id.into(),
            vehicle_class: cost_entity.vehicle_class.map(Into::into),
            vehicle_id: cost_entity.vehicle_id.map(Into::into),
            usd_cents: cost_entity.usd_cents,
            date: cost_entity.date.into(),
        }
    }
}
