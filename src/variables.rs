/// Represents a sensor's variable whose data to be retrieved from the experiment.
///
/// # Examples
///
/// ```
/// let sensor = Sensors::AccelerationX;
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Variables {
    MagnetometerX,
    MagnetometerY,
    MagnetometerZ,
    MagnetometerTime,

    GyroscopeX,
    GyroscopeY,
    GyroscopeZ,
    GyroscopeTime,

    AccelerationX,
    AccelerationY,
    AccelerationZ,
    AccelerationTime,

    Light,
    LightTime,
}

impl AsRef<str> for Variables {
    fn as_ref(&self) -> &str {
        match self {
            Variables::MagnetometerX => "magX",
            Variables::MagnetometerY => "magY",
            Variables::MagnetometerZ => "magZ",
            Variables::MagnetometerTime => "mag_time",

            Variables::GyroscopeX => "gyrX",
            Variables::GyroscopeY => "gyrY",
            Variables::GyroscopeZ => "gyrZ",
            Variables::GyroscopeTime => "gyr_time",

            Variables::AccelerationX => "accX",
            Variables::AccelerationY => "accY",
            Variables::AccelerationZ => "accZ",
            Variables::AccelerationTime => "acc_time",

            Variables::Light => "light",
            Variables::LightTime => "light_time",
        }
    }
}
