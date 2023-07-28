# phyphox-rust-api
Provides an easy access to the experiment data via sending HTTP requests under the hood.

Phyphox [official website](https://phyphox.org/)


_P.S. This code does not cover all sensors (i.e. temperature or lidar) because I was only able to know the names of the variables from the sensors that my device has. It's easy to add new sensors though. Create an issue if a sensor is lacking, but please provide the variable names for it._

# Examples

## Low-Level

```rust
// Create a client object
let mut phy = PhyphoxClient::new("192.168.0.1:8080");

// Register the variables to be read
phy.register(Variables::Light);
phy.register(Variables::MagnetometerX);

// Retreive the variable values
phy.retrieve().unwrap();

// Read the values
let light = phy.get(Variables::Light);
let magx = phy.get(Variables::MagnetometerX);

println!("{:?} {:?}", light, magx);
```

## High Level
```rust
// Create the low-level API client wrapper
let phy = Phyphox::new("192.168.0.1:8080");

// Declare what sensors you want to use
let light = phy.sv_sensor(SVSensorName::Light);
let magnetometer = phy.xyz_sensor(XYZSensorName::Magnetometer);

// Register the needed variables of the sensors so that they will be read
light.register();
magnetometer.register_x();

// Retreive the variable values
phy.retrieve().unwrap();

// Read the values
let light = light.value();
let magx = magnetometer.x();

println!("{:?} {:?}", light, magx);
```
