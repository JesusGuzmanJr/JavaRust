package club.marzipan.javacarrentals;

import java.util.UUID;

public class Vehicle {

    public enum VehicleClass {
        standard_small, standard_large, premium_small, premium_large
    }

    private UUID id;
    private String licensePlate;
    private String vin;
    private String year;
    private String make;
    private String model;
    private VehicleClass vehicleClass;

    public UUID getId() {
        return this.id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

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

    public String getYear() {
        return this.year;
    }

    public void setYear(String year) {
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
