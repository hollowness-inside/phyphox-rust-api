use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    sensors::{
        SVSensor,
        SVSensorName,
        XYZSensor,
        XYZSensorName,
    },
    Error,
    PhyphoxClient,
};

pub struct Phyphox(Rc<RefCell<PhyphoxClient>>);

/// Represents a wrapper around `PhyphoxClient`
impl Phyphox {
    /// Creates a wrapper around `PhyphoxClient`
    pub fn new(addr: &str) -> Self {
        Self(Rc::new(RefCell::new(PhyphoxClient::new(addr))))
    }

    /// Creates an `XYZSensor` object representing the requested sensor
    pub fn xyz_sensor(&self, sensor: XYZSensorName) -> XYZSensor {
        XYZSensor(sensor, self.0.clone())
    }

    /// Creates an `SVSensor` object representing the requested sensor
    pub fn sv_sensor(&self, sensor: SVSensorName) -> SVSensor {
        SVSensor(sensor, self.0.clone())
    }

    /// Retrieves data for all added variables and updates their values.
    ///
    /// # Errors
    ///
    /// Returns `Error::BadResponse` if the response is in the wrong format.
    /// 
    /// Returns `Error::ReqwestError` if the HTTP request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    ///
    /// let mag = phyphox.xyz_sensor(XYZSensorName::Magnetometer);
    /// mag.register_x();
    /// mag.register_y();
    ///
    /// loop {
    ///     phyphox.retrieve().unwrap();
    ///
    ///     let x = mag.x();
    ///     let y = mag.y();
    ///     println!("x = {:?}, y = {:?}", x, y);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// The `retrieve` function updates the values of all added variables. If a variable has not
    /// been added yet or its value could not be retrieved, its value will be `None`.
    pub fn retrieve(&self) -> Result<(), Error> {
        self.0.borrow_mut().retrieve()
    }

    /// Removes all the variables from the list to retrieve data for.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn clear_variables(&self) {
        self.0.borrow_mut().clear_variables()
    }

    /// Starts the experiment.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// If the experiment is already running, the function does nothing.
    pub fn start(&self) -> Result<(), Error> {
        self.0.borrow().start()
    }

    /// Stops the experiment.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// If the experiment is not running, the function does nothing.
    pub fn stop(&self) -> Result<(), Error> {
        self.0.borrow().stop()
    }

    /// Clears the experiment (and stops the experiment if running).
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    ///
    /// # Note
    ///
    /// The `clear` function sends a control command to the Phyphox experiment server
    /// to clear all buffer and will stop the experiment from running.
    pub fn clear(&self) -> Result<(), Error> {
        self.0.borrow().clear()
    }

    /// The `control` function sends a command directly to the Phyphox experiment server.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut phyphox = Phyphox::new("127.0.0.1:8080");
    /// phyphox.control("start")?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn control(&self, cmd: &str) -> Result<(), Error> {
        self.0.borrow().control(cmd)
    }
}
