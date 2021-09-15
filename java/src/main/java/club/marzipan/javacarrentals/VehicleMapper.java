package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class VehicleMapper {
    public Vehicle mapEntityToApi(VehicleEntity vehicleEntity) {
        if (vehicleEntity == null) {
            return null;
        }
        Vehicle vehicle = new Vehicle();
        vehicle.setId(vehicleEntity.getId());
        vehicle.setLicensePlate(vehicleEntity.getLicensePlate());
        vehicle.setVin(vehicleEntity.getVin());
        vehicle.setYear(vehicleEntity.getYear());
        vehicle.setMake(vehicleEntity.getMake());
        vehicle.setVehicleClass(mapEntityToApi(vehicleEntity.getVehicleClass()));
        return vehicle;
    }

    public Vehicle.VehicleClass mapEntityToApi(VehicleEntity.VehicleClass vehicleClass) {
        if (vehicleClass == null) {
            return null;
        }
        switch (vehicleClass) {
            case standard_small:
                return Vehicle.VehicleClass.standard_small;

            case standard_large:
                return Vehicle.VehicleClass.standard_large;

            case premium_small:
                return Vehicle.VehicleClass.premium_small;

            case premium_large:
                return Vehicle.VehicleClass.premium_large;

            default:
                return null;
        }
    }

    public VehicleEntity.VehicleClass mapApiToEntity(Vehicle.VehicleClass vehicleClass) {
        if (vehicleClass == null) {
            return null;
        }
        switch (vehicleClass) {
            case standard_small:
                return VehicleEntity.VehicleClass.standard_small;

            case standard_large:
                return VehicleEntity.VehicleClass.standard_large;

            case premium_small:
                return VehicleEntity.VehicleClass.premium_small;

            case premium_large:
                return VehicleEntity.VehicleClass.premium_large;

            default:
                return null;
        }
    }
}
