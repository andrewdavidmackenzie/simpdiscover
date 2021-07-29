use simpdiscoverylib::BeaconSender;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    println!("\nHit Control-C to kill the process and stop beacon sending\n");

    if let Ok(beacon) = BeaconSender::new(34254, "Hello") {
        beacon.send_loop()?;
    }

    Ok(())
}