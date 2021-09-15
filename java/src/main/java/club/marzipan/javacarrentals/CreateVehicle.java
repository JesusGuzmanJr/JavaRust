package club.marzipan.javacarrentals;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.Size;

import org.hibernate.validator.constraints.Range;

import club.marzipan.javacarrentals.Vehicle.VehicleClass;

public class CreateVehicle {

    @NotBlank
    @Size(min = 5, max = 7, message = "length must be 5 to 7 bytes")
    private String licensePlate;

    @NotBlank
    @Size(min = 17, max = 17, message = "length must be 17 bytes")
    private String vin;

    @Range(min = 2000, max = 2030, message = "year must be between 2000 and 2030 inclusive")
    private Integer year;

    @NotBlank(message = "make must not be blank")
    private String make;

    @NotBlank(message = "model must not be blank")
    private String model;

    @NotBlank(message = "model must not be blank")
    private VehicleClass vehicleClass;

    public String getLicensePlate() {
        return this.licensePlate;
    }

    public void setLicensePlate(String licensePlate) {
        this.licensePlate = licensePlate;
    }

    public String getVin() {
        return this.vin;
    }

    public void setVin(String vin) {
        this.vin = vin;
    }

    public Integer getYear() {
        return this.year;
    }

    public void setYear(Integer year) {
        this.year = year;
    }

    public String getMake() {
        return this.make;
    }

    public void setMake(String make) {
        this.make = make;
    }

    public String getModel() {
        return this.model;
    }

    public void setModel(String model) {
        this.model = model;
    }

    public VehicleClass getVehicleClass() {
        return this.vehicleClass;
    }

    public void setVehicleClass(VehicleClass vehicleClass) {
        this.vehicleClass = vehicleClass;
    }

}
