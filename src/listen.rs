use simpdiscoverylib::beacon_listener;
use simplog::simplog::SimpleLogger;

fn main() -> std::io::Result<()> {
    SimpleLogger::init_prefix(Some("info"), false);

    beacon_listener(34254)?;

    Ok(())
}
