use uuid::Uuid;

pub struct VehicleEntity {
    pub id: Uuid,
    pub license_plate: String,
    pub vin: String,
    pub year: i32,
    pub make: String,
    pub model: String,
    pub vehicle_class: VehicleClass,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "VEHICLE_CLASS", rename_all = "snake_case")]
pub enum VehicleClass {
    StandardSmall,
    StandardLarge,
    PremiumSmall,
    PremiumLarge,
}
