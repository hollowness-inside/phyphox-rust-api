use std::{
    thread,
    time::Duration,
};

use phyphox_api::{
    PhyphoxClient,
    Variables,
};

fn main() {
    // Create a client object
    let mut client = PhyphoxClient::new("192.168.0.1:8080");

    // Register the variables to be read
    client.register(Variables::Light);

    client.register(Variables::MagnetometerX);
    client.register(Variables::MagnetometerY);
    client.register(Variables::MagnetometerZ);

    // Run the experiment (and give some time for sensors to read some data)
    client.start().unwrap();
    thread::sleep(Duration::from_secs(1));

    // Retreive the variable values
    client.retrieve().unwrap();

    // Read the values
    let light = client.get(Variables::Light);
    let magx = client.get(Variables::MagnetometerX);
    let magy = client.get(Variables::MagnetometerY);
    let magz = client.get(Variables::MagnetometerZ);

    println!("{:?} {:?} {:?} {:?}", light, magx, magy, magz);

    // Stop the experiment
    client.stop().unwrap();
}
