use simpdiscoverylib::BeaconListener;
use simplog::simplog::SimpleLogger;
use std::time::Duration;

const BEACON_PORT : u16 = 9001;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    let args : Vec<String> = std::env::args().collect();
    let filter = match args.len() {
        0..=1 => None,
        _ => Some(args[1].clone())
    };

    let timeout = match args.len() {
        0..=2 => None,
        _ => {
            Some(Duration::from_secs(args[2].parse::<u64>().unwrap()))
        }
    };

    println!("Waiting for a beacon");
    if let Some(time) = timeout {
        println!("Timeout set to {} seconds", time.as_secs_f64());
    }

    if let Ok(listener) = BeaconListener::new(BEACON_PORT, filter.map(|f| f.into_bytes())) {
        println!("Beacon {}", listener.wait(timeout)?);
    }

    Ok(())
}
