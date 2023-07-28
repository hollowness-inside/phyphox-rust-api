/// Represents a sensor whose data can be retrieved from the experiment.
///
/// # Examples
///
/// ```
/// let sensor = Sensors::AccelerationX;
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sensors {
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

impl AsRef<str> for Sensors {
    fn as_ref(&self) -> &str {
        match self {
            Sensors::MagnetometerX => "magX",
            Sensors::MagnetometerY => "magY",
            Sensors::MagnetometerZ => "magZ",
            Sensors::MagnetometerTime => "mag_time",

            Sensors::GyroscopeX => "gyrX",
            Sensors::GyroscopeY => "gyrY",
            Sensors::GyroscopeZ => "gyrZ",
            Sensors::GyroscopeTime => "gyr_time",

            Sensors::AccelerationX => "accX",
            Sensors::AccelerationY => "accY",
            Sensors::AccelerationZ => "accZ",
            Sensors::AccelerationTime => "acc_time",

            Sensors::Light => "light",
            Sensors::LightTime => "light_time",
        }
    }
}
