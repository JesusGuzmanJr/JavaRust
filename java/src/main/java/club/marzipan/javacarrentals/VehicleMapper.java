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
        if (vehicleEntity.getVehicleClass() != null) {
            switch (vehicleEntity.getVehicleClass()) {
                case standard_small:
                    vehicle.setVehicleClass(Vehicle.VehicleClass.standard_small);
                    break;
                case standard_large:
                    vehicle.setVehicleClass(Vehicle.VehicleClass.standard_large);
                    break;
                case premium_small:
                    vehicle.setVehicleClass(Vehicle.VehicleClass.premium_small);
                    break;
                case premium_large:
                    vehicle.setVehicleClass(Vehicle.VehicleClass.premium_large);
                    break;
            }
        }
        return vehicle;
    }
}
