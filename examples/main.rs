use phyphox_api::{
    Error,
    Phyphox,
    Variables,
};

fn main() -> Result<(), Error> {
    let mut phy = Phyphox::new("127.0.0.1:8080");

    phy.register(Variables::MagnetometerX);
    phy.register(Variables::Light);

    loop {
        phy.retrieve()?;

        let m = phy.get(Variables::MagnetometerX).unwrap();
        let l = phy.get(Variables::Light).unwrap();

        println!("{} {}", m, l);
    }
}
