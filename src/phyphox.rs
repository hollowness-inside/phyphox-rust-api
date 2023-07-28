use std::collections::HashMap;

use crate::{
    error::Error,
    variables::Variables,
};

/// Represents a connection to the Phyphox experiment server.
pub struct Phyphox {
    url: String,
    variables: HashMap<Variables, Option<f64>>,
}

impl Phyphox {
    /// Creates a new Phyphox client
    pub fn new<S: std::fmt::Display>(addr: S) -> Self {
        Self {
            url: format!("http://{}", addr),
            variables: HashMap::new(),
        }
    }

    /// Adds a sensor's variable to the list to retrieve data for.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to add to the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    /// phyphox.register(Variables::AccelerationX);
    /// phyphox.register(Variables::AccelerationY);
    /// ```
    #[inline]
    pub fn register(&mut self, variable: Variables) {
        self.variables.insert(variable, None);
    }

    /// Retrieves data for all added variables and updates their values.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    /// phyphox.register(Variables::AccelerationX);
    /// phyphox.register(Variables::AccelerationY);
    /// phyphox.start()?;
    ///
    /// loop {
    ///     phyphox.retrieve()?;
    ///
    ///     let x = phyphox.get(Variables::AccelerationX)?;
    ///     let y = phyphox.get(Variables::AccelerationY)?;
    ///     println!("x = {}, y = {}", x, y);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// The `retrieve` function updates the values of all added variables. If a variable has not
    /// been added yet or its value could not be retrieved, its value will be `None`.
    pub fn retrieve(&mut self) -> Result<(), Error> {
        let variables: Vec<Variables> = self.variables.keys().map(|x| *x).collect();

        let var_names: Vec<&str> = variables.iter().map(|s| s.as_ref()).collect();
        let url = format!("{}/get?{}", self.url, var_names.join("&"));
        let response = reqwest::blocking::get(&url)?;

        let json = response.json::<serde_json::Value>()?;
        let json = json.get("buffer").ok_or(Error::BadResponse)?;

        for var in variables {
            json.get(var.as_ref())
                .and_then(|v| v.get("buffer"))
                .and_then(|v| v.get(0))
                .and_then(|v| self.variables.insert(var, v.as_f64()));
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
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    /// phyphox.register(Variables::AccelerationX);
    /// phyphox.retrieve().unwrap();
    ///
    /// let x = phyphox.get(Variables::AccelerationX).unwrap();
    /// println!("x = {}", x);
    /// ```
    #[inline]
    pub fn get(&self, sensor: Variables) -> Option<f64> {
        self.variables.get(&sensor).and_then(|v| *v)
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
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
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
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
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
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
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
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
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

    /// Removes all the variables from the list to retrieve data for.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    /// phyphox.clear_variables().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    #[inline]
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }
}
