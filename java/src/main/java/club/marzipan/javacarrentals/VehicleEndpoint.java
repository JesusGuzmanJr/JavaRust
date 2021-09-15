package club.marzipan.javacarrentals;

import java.util.Optional;
import java.util.UUID;

import javax.validation.Valid;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

import club.marzipan.javacarrentals.VehicleEntity.VehicleClass;

@RestController
public class VehicleEndpoint {

    @Autowired
    private VehicleRepository vehicleRepository;

    @Autowired
    private VehicleMapper vehicleMapper;

    @GetMapping("/vehicle/{id}")
    public ResponseEntity<Vehicle> getVehicle(@PathVariable UUID id) {
        Optional<VehicleEntity> vehicleEntity = vehicleRepository.findById(id);
        if (vehicleEntity.isPresent()) {
            Vehicle vehicle = vehicleMapper.mapEntityToApi(vehicleEntity.get());
            return ResponseEntity.ok().body(vehicle);
        } else {
            return ResponseEntity.notFound().build();
        }
    }

    @PostMapping("/vehicle")
    public ResponseEntity<Vehicle> createVehicle(@Valid @RequestBody CreateVehicle createVehicle) {

        VehicleEntity vehicleEntity = new VehicleEntity();
        vehicleEntity.setId(UUID.randomUUID());
        vehicleEntity.setLicensePlate(createVehicle.getLicensePlate());
        vehicleEntity.setVin(createVehicle.getVin());
        vehicleEntity.setYear(createVehicle.getYear());
        vehicleEntity.setMake(createVehicle.getMake());
        vehicleEntity.setModel(createVehicle.getModel());
        vehicleEntity.setVehicleClass(VehicleClass.premium_large);

        vehicleRepository.saveAndFlush(vehicleEntity);
        return ResponseEntity.ok().body(vehicleMapper.mapEntityToApi(vehicleEntity));
    }

}
