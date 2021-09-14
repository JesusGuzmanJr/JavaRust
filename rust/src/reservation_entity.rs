use uuid::Uuid;

use crate::date_time::DateTime;

pub struct ReservationEntity {
    pub id: Uuid,
    pub account_id: Uuid,
    pub vehicle_id: Uuid,
    pub start_date_time: DateTime,
    pub end_date_time: DateTime,
}
