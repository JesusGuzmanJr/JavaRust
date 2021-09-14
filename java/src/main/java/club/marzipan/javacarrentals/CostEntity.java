package club.marzipan.javacarrentals;

import java.time.ZonedDateTime;
import java.util.UUID;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.EnumType;
import javax.persistence.Enumerated;
import javax.persistence.Id;
import javax.persistence.Table;

import club.marzipan.javacarrentals.VehicleEntity.VehicleClass;

@Entity
@Table(name = "cost")
public class CostEntity {

    @Id
    private UUID id;

    @Column(name = "vehicle_class")
    @Enumerated(EnumType.STRING)
    private VehicleClass vehicleClass;

    @Column(name = "vehicle_id")
    private UUID vehicleId;

    @Column(name = "usd_cents")
    private Integer usdCents;

    @Column(name = "date")
    private ZonedDateTime date;

    public UUID getId() {
        return this.id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public VehicleClass getVehicleClass() {
        return this.vehicleClass;
    }

    public void setVehicleClass(VehicleClass vehicleClass) {
        this.vehicleClass = vehicleClass;
    }

    public UUID getVehicleId() {
        return this.vehicleId;
    }

    public void setVehicleId(UUID vehicleId) {
        this.vehicleId = vehicleId;
    }

    public Integer getUsdCents() {
        return this.usdCents;
    }

    public void setUsdCents(Integer usdCents) {
        this.usdCents = usdCents;
    }

    public ZonedDateTime getDate() {
        return this.date;
    }

    public void setDate(ZonedDateTime date) {
        this.date = date;
    }

}
