package club.marzipan.javacarrentals;

import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;

public interface VehicleRepository extends JpaRepository<VehicleEntity, UUID> {

}
