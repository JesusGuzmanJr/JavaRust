use sqlx::{error::Error, postgres::PgQueryResult};

use crate::{
    persistance,
    vehicle::Id,
    vehicle_entity::{VehicleClass, VehicleEntity},
};

pub async fn find_by_id(id: Id) -> Result<Option<VehicleEntity>, Error> {
    sqlx::query_as!(
        VehicleEntity,
        r#"SELECT id, license_plate, vin, year, make, model, vehicle_class AS "vehicle_class!: VehicleClass" FROM vehicle WHERE id = $1"#,
        id.0
    )
    .fetch_optional(persistance::database_connector())
    .await
}

pub async fn save_and_flush(vehicle_entity: &VehicleEntity) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"INSERT INTO vehicle VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        vehicle_entity.id,
        vehicle_entity.license_plate,
        vehicle_entity.vin,
        vehicle_entity.year,
        vehicle_entity.make,
        vehicle_entity.model,
        &vehicle_entity.vehicle_class as &VehicleClass,
    )
    .execute(persistance::database_connector())
    .await
}
