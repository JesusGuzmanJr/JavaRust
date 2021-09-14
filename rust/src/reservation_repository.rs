use sqlx::{error::Error, postgres::PgQueryResult};

use crate::{persistance, reservation::Id, reservation_entity::ReservationEntity};

pub async fn find_by_id(id: Id) -> Result<Option<ReservationEntity>, Error> {
    sqlx::query_as!(
        ReservationEntity,
        r#"SELECT id, account_id, vehicle_id, start_date_time, end_date_time FROM reservation WHERE id = $1"#,
        id.0
    )
    .fetch_optional(persistance::database_connector())
    .await
}

pub async fn save_and_flush(
    reservation_entity: &ReservationEntity,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"INSERT INTO reservation VALUES ($1, $2, $3, $4, $5)"#,
        reservation_entity.id,
        reservation_entity.account_id,
        reservation_entity.vehicle_id,
        reservation_entity.start_date_time,
        reservation_entity.end_date_time,
    )
    .execute(persistance::database_connector())
    .await
}
