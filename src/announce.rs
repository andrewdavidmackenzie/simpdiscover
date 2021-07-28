use simpdiscoverylib::Beacon;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    if let Ok(beacon) = Beacon::new(34254) {
        beacon.send_loop()?;
    }

    Ok(())
}
