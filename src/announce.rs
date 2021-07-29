use simpdiscoverylib::BeaconSender;
use simplog::simplog::SimpleLogger;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    println!("\nHit Control-C to kill the process and stop beacon sending\n");

    let args : Vec<String> = std::env::args().collect();
    let message = match args.len() {
        0..=1 => "Hello",
        _ => &args[1]
    };

    println!("Beacon message set to: '{}'", message);

    if let Ok(beacon) = BeaconSender::new(34254, message) {
        beacon.send_loop(Duration::from_secs(1))?;
    }

    Ok(())
}