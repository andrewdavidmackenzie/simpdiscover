use simpdiscoverylib::BeaconSender;
use simpdiscoverylib::beacon_listener;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    if let Ok(beacon) = BeaconSender::new(34254) {
        std::thread::spawn(move || {
            let _ = beacon.send_loop();
        });
    }

    let beacon = beacon_listener(34254)?;
    println!("Beacon with message '{}' received from IP: {}", beacon.message, beacon.source_ip);

    Ok(())
}
