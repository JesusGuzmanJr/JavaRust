package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class ReservationMapper {
    public Reservation mapEntityToApi(ReservationEntity reservationEntity) {
        if (reservationEntity == null) {
            return null;
        }
        Reservation reservation = new Reservation();
        reservation.setId(reservationEntity.getId());
        reservation.setAccountId(reservationEntity.getAccountId());
        reservation.setVehicleId(reservationEntity.getVehicleId());
        reservation.setStartDateTime(reservationEntity.getStartDateTime());
        reservation.setEndDateTime(reservationEntity.getEndDateTime());
        return reservation;
    }
}
