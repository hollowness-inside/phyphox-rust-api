# phyphox-rust-api
Provides an easy access to the experiment data via sending HTTP requests under the hood.

Phyphox [official website](https://phyphox.org/)


_P.S. This code does not cover all sensors (i.e. temperature or lidar) because I was only able to know the names of the variables from the sensors that my device has. It's easy to add new sensors though. Create an issue if a sensor is lacking, but please provide the variable names for it._

# Example

```rust
// Create an experiment client
let phy = Phyhpox::new("host:port");

// Register variables to be read
phy.add_sensor(Sensors::MagnetometerX);
phy.add_sensor(Sensors::MagnetometerY);
phy.add_sensor(Sensors::MagnetometerZ);

// Retrieve the variables' values
phy.retrieve_data()?;

// Read the variables' values
let magx = phy.get(Sensors::MagnetometerX)?;
let magy = phy.get(Sensors::MagnetometerY)?;
let magz = phy.get(Sensors::MagnetometerZ)?;
```
