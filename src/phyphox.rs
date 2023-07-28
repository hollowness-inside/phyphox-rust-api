use std::collections::HashMap;

use crate::{
    error::Error,
    sensors::Sensors,
};

/// Represents a connection to the Phyphox experiment server.
pub struct Phyphox {
    url: String,
    sensors: HashMap<Sensors, Option<f64>>,
}

impl Phyphox {
    /// Creates a new Phyphox client
    pub fn new<S: std::fmt::Display>(addr: S) -> Self {
        Self {
            url: format!("http://{}", addr),
            sensors: HashMap::new(),
        }
    }

    /// Adds a sensor to the list of sensors to retrieve data for.
    ///
    /// # Arguments
    ///
    /// * `sensor` - The sensor to add to the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.add_sensor(Sensors::AccelerationX);
    /// phyphox.add_sensor(Sensors::AccelerationY);
    /// ```
    #[inline]
    pub fn add_sensor(&mut self, sensor: Sensors) {
        self.sensors.insert(sensor, None);
    }

    /// Retrieves data for all added sensors and updates their values.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.add_sensor(Sensors::AccelerationX);
    /// phyphox.add_sensor(Sensors::AccelerationY);
    /// phyphox.start().unwrap();
    ///
    /// loop {
    ///     phyphox.retrieve_data().unwrap();
    ///     let x = phyphox.get(Sensors::AccelerationX).unwrap();
    ///     let y = phyphox.get(Sensors::AccelerationY).unwrap();
    ///     println!("x = {}, y = {}", x, y);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// The `retrieve_data` function updates the values of all added sensors. If a sensor has not
    /// been added yet or it could not be retrieved, its value will be `None`.
    pub fn retrieve_data(&mut self) -> Result<(), Error> {
        let sensors: Vec<Sensors> = self.sensors.keys().map(|x| *x).collect();

        let sensor_names: Vec<&str> = sensors.iter().map(|s| s.as_ref()).collect();
        let url = format!("{}/get?{}", self.url, sensor_names.join("&"));
        let response = reqwest::blocking::get(&url)?;

        let json = response.json::<serde_json::Value>()?;
        let json = json.get("buffer").ok_or(Error::BadResponse)?;

        for sensor in sensors {
            json.get(sensor.as_ref())
                .and_then(|v| v.get("buffer"))
                .and_then(|v| v.get(0))
                .and_then(|v| self.sensors.insert(sensor, v.as_f64()));
        }

        Ok(())
    }

    /// Returns the value of a sensor.
    ///
    /// # Arguments
    ///
    /// * `sensor` - The sensor to get the value of.
    ///
    /// # Returns
    ///
    /// Returns the value of the sensor as a `Option<f64>`. If the sensor has not been added
    /// or if its value could not be retrieved, the function will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.add_sensor(Sensors::AccelerationX);
    /// phyphox.retrieve_data().unwrap();
    ///
    /// let x = phyphox.get(Sensors::AccelerationX).unwrap();
    /// println!("x = {}", x);
    /// ```
    #[inline]
    pub fn get(&self, sensor: Sensors) -> Option<f64> {
        self.sensors.get(&sensor).and_then(|v| *v)
    }

    /// The `control` function sends a command directly to the Phyphox experiment server.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to send.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.control("start").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn control(&self, command: &str) -> Result<(), Error> {
        let url = format!("{}/control?cmd={}", self.url, command);
        let _ = reqwest::blocking::get(&url)?;
        Ok(())
    }

    /// Stops the experiment.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.start().unwrap();
    /// // Retrieve some measurements...
    /// phyphox.stop().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// If the experiment is not running, the function does nothing.
    #[inline]
    pub fn stop(&self) -> Result<(), Error> {
        self.control("stop")
    }

    /// Starts the experiment.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.start().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// If the experiment is already running, the function does nothing.
    #[inline]
    pub fn start(&self) -> Result<(), Error> {
        self.control("start")
    }

    /// Clears the experiment (and stops the experiment if running).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.clear().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// The `clear` function sends a control command to the Phyphox experiment server
    /// to clear all buffer and will stop the experiment from running.
    #[inline]
    pub fn clear(&self) -> Result<(), Error> {
        self.control("clear")
    }

    /// Removes all the sensors from the list of sensors to retrieve data for.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("192.168.1.100");
    /// phyphox.reset_sensors().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    #[inline]
    pub fn reset_sensors(&mut self) {
        self.sensors.clear();
    }
}
