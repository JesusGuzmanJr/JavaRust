use sqlx::{error::Error, postgres::PgQueryResult};

use crate::{cost::Id, cost_entity::CostEntity, persistance, vehicle_entity::VehicleClass};

pub async fn find_by_id(id: Id) -> Result<Option<CostEntity>, Error> {
    sqlx::query_as!(
        CostEntity,
        r#"SELECT id, vehicle_class AS "vehicle_class?: VehicleClass", vehicle_id, usd_cents, date FROM cost WHERE id = $1"#,
        id.0
    )
    .fetch_optional(persistance::database_connector())
    .await
}

pub async fn save_and_flush(cost_entity: &CostEntity) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"INSERT INTO cost VALUES ($1, $2, $3, $4, $5)"#,
        cost_entity.id,
        &cost_entity.vehicle_class as &Option<VehicleClass>,
        cost_entity.vehicle_id,
        cost_entity.usd_cents,
        cost_entity.date,
    )
    .execute(persistance::database_connector())
    .await
}
