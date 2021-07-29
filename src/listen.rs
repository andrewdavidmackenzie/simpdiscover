use simpdiscoverylib::BeaconListener;
use simplog::simplog::SimpleLogger;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    let args : Vec<String> = std::env::args().collect();
    let filter = match args.len() {
        0..=1 => None,
        _ => Some(args[1].clone())
    };

    if let Ok(listener) = BeaconListener::new(34254, filter) {
        let beacon = listener.wait(Some(Duration::from_secs(5)))?;
        println!("Beacon with message '{}' received from IP: {}, port: {}",
                 beacon.message,
                 beacon.source_ip,
                 beacon.source_port);
    }

    Ok(())
}
