use crate::{
    vehicle::{Vehicle, VehicleClass},
    vehicle_entity::{VehicleClass as VehicleClassEntity, VehicleEntity},
};

impl From<VehicleEntity> for Vehicle {
    fn from(vehicle_entity: VehicleEntity) -> Vehicle {
        Vehicle {
            id: vehicle_entity.id.into(),
            license_plate: vehicle_entity.license_plate.into(),
            vin: vehicle_entity.vin.into(),
            year: vehicle_entity.year as _,
            make: vehicle_entity.make.into(),
            model: vehicle_entity.model.into(),
            vehicle_class: vehicle_entity.vehicle_class.into(),
        }
    }
}

impl From<VehicleClassEntity> for VehicleClass {
    fn from(status: VehicleClassEntity) -> VehicleClass {
        match status {
            VehicleClassEntity::StandardSmall => VehicleClass::StandardSmall,
            VehicleClassEntity::StandardLarge => VehicleClass::StandardLarge,
            VehicleClassEntity::PremiumSmall => VehicleClass::PremiumSmall,
            VehicleClassEntity::PremiumLarge => VehicleClass::PremiumLarge,
        }
    }
}
