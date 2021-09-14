use derive_more::{Display, From, Into};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: Id,
    pub license_plate: LicensePlate,
    pub vin: Vin,
    pub year: u32,
    pub make: Make,
    pub model: Model,
    pub vehicle_class: VehicleClass,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, From, Into, Display)]
pub struct Id(pub uuid::Uuid);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct LicensePlate(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct Vin(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct Make(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct Model(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Display)]
#[serde(rename_all = "lowercase")]
pub enum VehicleClass {
    StandardSmall,
    StandardLarge,
    PremiumSmall,
    PremiumLarge,
}
