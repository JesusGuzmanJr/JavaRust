use uuid::Uuid;

use crate::{date_time::DateTime, vehicle_entity::VehicleClass};

pub struct CostEntity {
    pub id: Uuid,
    pub vehicle_class: Option<VehicleClass>,
    pub vehicle_id: Option<Uuid>,
    pub usd_cents: i32,
    pub date: DateTime,
}
