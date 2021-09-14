use crate::{account::Id as AccountId, date_time::DateTime, vehicle::Id as VehicleId};
use derive_more::{Display, From, Into};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reservation {
    pub id: Id,
    pub account_id: AccountId,
    pub vehicle_id: VehicleId,
    pub start_date_time: DateTime,
    pub end_date_time: DateTime,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, From, Into, Display)]
pub struct Id(pub uuid::Uuid);
