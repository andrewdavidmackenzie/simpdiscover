use simpdiscoverylib::beacon_listener;

fn main() -> std::io::Result<()> {
    beacon_listener(34254)?;

    Ok(())
}
