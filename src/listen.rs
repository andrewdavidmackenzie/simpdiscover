use simpdiscoverylib::BeaconListener;
use env_logger::Builder;
use std::time::Duration;
use log::LevelFilter;

const BEACON_TEST_SERVICE_NAME :&str = "BeaconTestService";

fn main() -> std::io::Result<()> {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Info).init();

    let args : Vec<String> = std::env::args().collect();
    let service_name = match args.len() {
        0..=1 => BEACON_TEST_SERVICE_NAME.to_string(),
        _ => args[1].clone()
    };

    let timeout = match args.len() {
        1 => None,
        _ => Some(Duration::from_secs(args[2].parse::<u64>().unwrap()))
    };

    println!("Timeout set to {:?}", timeout);

    println!("Waiting for a beacon from service: '{}'", service_name);
    let listener = BeaconListener::new(service_name.as_bytes(), 9002)?;
    println!("Beacon {}", listener.wait(timeout)?);

    Ok(())
}
