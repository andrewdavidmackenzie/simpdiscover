use simpdiscoverylib::BeaconSender;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    if let Ok(beacon) = BeaconSender::new(34254) {
        beacon.send_loop()?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use simpdiscoverylib::BeaconSender;
    use simpdiscoverylib::beacon_listener;

    #[test]
    fn beacon_is_received() {
        let port = 34254;
        if let Ok(beacon) = BeaconSender::new(port) {
            std::thread::spawn(move || {
                let _ = beacon.send_loop();
            });
        }

        let beacon = beacon_listener(port).expect("Could not listen for beacon");
        assert_eq!(beacon.message, "Hello");
    }
}
