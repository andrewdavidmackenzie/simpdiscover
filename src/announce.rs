use simpdiscoverylib::BeaconSender;
use simplog::SimpleLogger;
use std::time::Duration;

const BEACON_TEST_SERVICE_PORT : u16 = 15002;
const BEACON_TEST_SERVICE_NAME :&str = "BeaconTestService";

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    println!("\nHit Control-C to kill the process and stop beacon sending\n");

    let args : Vec<String> = std::env::args().collect();
    let service_name = match args.len() {
        0..=1 => BEACON_TEST_SERVICE_NAME,
        _ => &args[1]
    };

    println!("Beacon message set to: '{}'", service_name);

    if let Ok(beacon) = BeaconSender::new(BEACON_TEST_SERVICE_PORT,
                                          service_name.as_bytes(), 9002) {
        beacon.send_loop(Duration::from_secs(1))?;
    }

    Ok(())
}