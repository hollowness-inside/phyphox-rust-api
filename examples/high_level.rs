use std::{
    thread,
    time::Duration,
};

use phyphox_api::{
    sensors::{
        SVSensorName,
        XYZSensorName,
    },
    Phyphox,
};

fn main() {
    // Create the low-level API client wrapper
    let phy = Phyphox::new("192.168.0.1:8080");

    // Declare what sensors you want to use
    let light = phy.sv_sensor(SVSensorName::Light);
    let magnetometer = phy.xyz_sensor(XYZSensorName::Magnetometer);

    // Register the needed variables of the sensors so that they will be read
    light.register();
    magnetometer.register_x();
    magnetometer.register_y();
    // magnetometer.register_z();

    // Run the experiment (and give some time for sensors to read some data)
    phy.start().unwrap();
    thread::sleep(Duration::from_secs(1));

    // Retreive the variable values
    phy.retrieve().unwrap();

    // Read the data
    let light = light.value();
    let magx = magnetometer.x();
    let magy = magnetometer.y();
    let magz = magnetometer.z();

    println!("{:?} {:?} {:?} {:?}", light, magx, magy, magz);

    // Stop the experiment
    phy.stop().unwrap();
}
