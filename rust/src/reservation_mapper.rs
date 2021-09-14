use crate::{reservation::Reservation, reservation_entity::ReservationEntity};

impl From<ReservationEntity> for Reservation {
    fn from(reservation_entity: ReservationEntity) -> Reservation {
        Reservation {
            id: reservation_entity.id.into(),
            account_id: reservation_entity.account_id.into(),
            vehicle_id: reservation_entity.vehicle_id.into(),
            start_date_time: reservation_entity.start_date_time.into(),
            end_date_time: reservation_entity.end_date_time.into(),
        }
    }
}
