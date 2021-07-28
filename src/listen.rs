use simpdiscoverylib::BeaconListener;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    if let Ok(listener) = BeaconListener::new(34254) {
        let beacon = listener.wait()?;
        println!("Beacon with message '{}' received from IP: {}", beacon.message, beacon.source_ip);
    }

    Ok(())
}
