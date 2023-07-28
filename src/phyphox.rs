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

impl Phyphox {
    pub fn new(addr: &str) -> Self {
        Self(Rc::new(RefCell::new(PhyphoxClient::new(addr))))
    }

    pub fn retrieve(&self) -> Result<(), Error> {
        self.0.borrow_mut().retrieve()
    }

    pub fn clear_variables(&self) {
        self.0.borrow_mut().clear_variables()
    }

    pub fn start(&self) -> Result<(), Error> {
        self.0.borrow().start()
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.0.borrow().stop()
    }

    pub fn clear(&self) -> Result<(), Error> {
        self.0.borrow().clear()
    }

    pub fn xyz_sensor(&self, sensor: XYZSensorName) -> XYZSensor {
        XYZSensor(sensor, self.0.clone())
    }

    pub fn sv_sensor(&self, sensor: SVSensorName) -> SVSensor {
        SVSensor(sensor, self.0.clone())
    }
}
