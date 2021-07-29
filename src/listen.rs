use simpdiscoverylib::BeaconListener;
use simplog::simplog::SimpleLogger;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    if let Ok(listener) = BeaconListener::new(34254) {
        let beacon = listener.wait(Some(Duration::from_secs(5)))?;
        println!("Beacon with message '{}' received from IP: {}", beacon.message, beacon.source_ip);
    }

    Ok(())
}
