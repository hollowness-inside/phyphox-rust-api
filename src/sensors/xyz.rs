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
        #[doc = "Registers the "]
        #[doc = stringify!($var)]
        #[doc = " variable to be read"]
        pub fn $fn_name(&self) {
            self.1.borrow_mut().register(self.0.$var())
        }
    };
}

macro_rules! getter {
    ($var:ident) => {
        #[doc = "Reads the "]
        #[doc = stringify!($var)]
        #[doc = " variable from the sensor"]
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
    /// Returns the `Variable` representing the X value of this sensor
    pub fn x(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerX,
            XYZSensorName::Acceleration => Variables::AccelerationX,
            XYZSensorName::Gyroscope => Variables::GyroscopeX,
        }
    }

    /// Returns the `Variable` representing the Y value of this sensor
    pub fn y(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerY,
            XYZSensorName::Acceleration => Variables::AccelerationY,
            XYZSensorName::Gyroscope => Variables::GyroscopeY,
        }
    }

    /// Returns the `Variable` representing the Z value of this sensor
    pub fn z(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerZ,
            XYZSensorName::Acceleration => Variables::AccelerationZ,
            XYZSensorName::Gyroscope => Variables::GyroscopeZ,
        }
    }

    /// Returns the `Variable` representing the Abs value of this sensor
    pub fn abs(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerAbs,
            XYZSensorName::Acceleration => Variables::AccelerationAbs,
            XYZSensorName::Gyroscope => Variables::GyroscopeAbs,
        }
    }

    /// Returns the `Variable` representing the Time value of this sensor
    pub fn time(&self) -> Variables {
        match self {
            XYZSensorName::Magnetometer => Variables::MagnetometerTime,
            XYZSensorName::Acceleration => Variables::AccelerationTime,
            XYZSensorName::Gyroscope => Variables::GyroscopeTime,
        }
    }
}

/// Represents an XYZ sensor
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
