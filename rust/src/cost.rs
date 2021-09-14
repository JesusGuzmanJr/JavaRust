use crate::{
    date_time::DateTime,
    vehicle::{Id as VehicleId, VehicleClass},
};
use derive_more::{Display, From, Into};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub id: Id,
    pub vehicle_class: Option<VehicleClass>,
    pub vehicle_id: Option<VehicleId>,
    pub usd_cents: i32,
    pub date: DateTime,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, From, Into, Display)]
pub struct Id(pub uuid::Uuid);
