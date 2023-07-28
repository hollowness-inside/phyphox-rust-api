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
    pub fn value(&self) -> Variables {
        match self {
            SVSensorName::Light => Variables::Light,
        }
    }

    pub fn time(&self) -> Variables {
        match self {
            SVSensorName::Light => Variables::LightTime,
        }
    }
}

pub struct SVSensor(
    pub(crate) SVSensorName,
    pub(crate) Rc<RefCell<PhyphoxClient>>,
);

impl SVSensor {
    pub fn register(&self) {
        self.1.borrow_mut().register(self.0.value())
    }

    pub fn value(&self) -> Option<f64> {
        self.1.borrow().get(self.0.value())
    }

    pub fn time(&self) -> Option<f64> {
        self.1.borrow().get(self.0.time())
    }
}
