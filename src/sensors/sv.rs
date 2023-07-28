use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    PhyphoxClient,
    Variables,
};

pub enum SVSensorName {
    Light,
}

impl SVSensorName {
    /// Returns the `value` variable of the this sensor as a `Variable`
    pub fn value(&self) -> Variables {
        match self {
            SVSensorName::Light => Variables::Light,
        }
    }

    /// Returns the `time` variable of the this sensor as a `Variable`
    pub fn time(&self) -> Variables {
        match self {
            SVSensorName::Light => Variables::LightTime,
        }
    }
}

/// Represents a single value sensor
pub struct SVSensor(
    pub(crate) SVSensorName,
    pub(crate) Rc<RefCell<PhyphoxClient>>,
);

impl SVSensor {
    /// Registers the `value` variable to be read
    pub fn register(&self) {
        self.1.borrow_mut().register(self.0.value())
    }

    /// Registers the `time` variable to be read
    pub fn register_time(&self) {
        self.1.borrow_mut().register(self.0.time())
    }

    /// Reads the `value` variable from this sensor
    pub fn value(&self) -> Option<f64> {
        self.1.borrow().get(self.0.value())
    }

    /// Reads the `time` variable from this sensor
    pub fn time(&self) -> Option<f64> {
        self.1.borrow().get(self.0.time())
    }
}
