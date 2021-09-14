package club.marzipan.javacarrentals;

import java.time.ZonedDateTime;
import java.util.UUID;

import club.marzipan.javacarrentals.Vehicle.VehicleClass;

public class Cost {

    private UUID id;
    private VehicleClass vehicleClass;
    private UUID vehicleId;
    private Integer UsdCents;
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
        return this.UsdCents;
    }

    public void setUsdCents(Integer UsdCents) {
        this.UsdCents = UsdCents;
    }

    public ZonedDateTime getDate() {
        return this.date;
    }

    public void setDate(ZonedDateTime date) {
        this.date = date;
    }

}
