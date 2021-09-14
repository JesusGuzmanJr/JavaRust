package club.marzipan.javacarrentals;

import org.springframework.stereotype.Component;

@Component
public class CostMapper {
    public Cost mapEntityToApi(CostEntity costEntity) {
        if (costEntity == null) {
            return null;
        }
        Cost cost = new Cost();
        cost.setId(costEntity.getId());
        if (costEntity.getVehicleClass() != null) {
            switch (costEntity.getVehicleClass()) {
                case standard_small:
                    cost.setVehicleClass(Vehicle.VehicleClass.standard_small);
                    break;
                case standard_large:
                    cost.setVehicleClass(Vehicle.VehicleClass.standard_large);
                    break;
                case premium_small:
                    cost.setVehicleClass(Vehicle.VehicleClass.premium_small);
                    break;
                case premium_large:
                    cost.setVehicleClass(Vehicle.VehicleClass.premium_large);
                    break;
            }
        }
        cost.setVehicleId(costEntity.getVehicleId());
        cost.setUsdCents(costEntity.getUsdCents());
        cost.setDate(costEntity.getDate());
        return cost;
    }
}
