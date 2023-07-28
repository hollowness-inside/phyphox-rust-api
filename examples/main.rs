use phyphox_api::{
    Error,
    Phyphox,
    Sensors,
};

fn main() -> Result<(), Error> {
    let mut phy = Phyphox::new("127.0.0.1:8080");

    phy.add_sensor(Sensors::MagnetometerX);
    phy.add_sensor(Sensors::Light);

    loop {
        phy.retrieve_data()?;

        let m = phy.get(Sensors::MagnetometerX).expect("cannot retrieve magX");
        let l = phy.get(Sensors::Light).expect("cannot retrieve light");

        println!("{} {}", m, l);
    }
}
