use simpdiscoverylib::BeaconListener;
use simplog::SimpleLogger;
use std::time::Duration;

const BEACON_TEST_SERVICE_NAME :&str = "BeaconTestService";

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

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
