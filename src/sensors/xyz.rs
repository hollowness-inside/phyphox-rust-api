use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    PhyphoxClient,
    Variables,
};

macro_rules! register {
    ($fn_name:ident, $var:ident) => {
        pub fn $fn_name(&self) {
            self.1.borrow_mut().register(self.0.$var())
        }
    };
}

macro_rules! getter {
    ($var:ident) => {
        pub fn $var(&self) -> Option<f64> {
            self.1.borrow().get(self.0.$var())
        }
    };
}

pub enum XYZSensorName {
    Magnetometer,
    Acceleration,
    Gyroscope,
}

impl XYZSensorName {
    pub fn x(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerX,
            XYZSensorName::Acceleration => Variables::AccelerationX,
            XYZSensorName::Gyroscope => Variables::GyroscopeX,
        }
    }

    pub fn y(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerY,
            XYZSensorName::Acceleration => Variables::AccelerationY,
            XYZSensorName::Gyroscope => Variables::GyroscopeY,
        }
    }

    pub fn z(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerZ,
            XYZSensorName::Acceleration => Variables::AccelerationZ,
            XYZSensorName::Gyroscope => Variables::GyroscopeZ,
        }
    }

    pub fn abs(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerAbs,
            XYZSensorName::Acceleration => Variables::AccelerationAbs,
            XYZSensorName::Gyroscope => Variables::GyroscopeAbs,
        }
    }

    pub fn time(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerTime,
            XYZSensorName::Acceleration => Variables::AccelerationTime,
            XYZSensorName::Gyroscope => Variables::GyroscopeTime,
        }
    }
}

pub struct XYZSensor(
    pub(crate) XYZSensorName,
    pub(crate) Rc<RefCell<PhyphoxClient>>,
);

impl XYZSensor {
    register!(register_x, x);
    register!(register_y, y);
    register!(register_z, z);
    register!(register_abs, abs);
    register!(register_time, time);

    getter!(x);
    getter!(y);
    getter!(z);
    getter!(abs);
    getter!(time);
}
