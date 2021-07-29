# Simpdiscovery

Simpdiscovery is a simple rust crate to facilitate discovery of services within the Local Area network (LAN) 
using UDP Broadcast datagrams or "beacons".

The 'announcer' of a service could be a process on another machines, or another threads or process on the same machine.

# Goals
* LAN wide detection of named services and what IP they are at
* Simple to understand and use
* Small, few dependencies on the library and small memory and cpu footprint
* Simple beacon format that is easy to use and that doesn't introduce specific file format support into the library

# Non-Goals
* Discovery of services across LANs, WANs, the Web or in the cloud.
  
# Implemented so far
* BeaconSender struct that can be setup to send beacons:
  * with a specific beacon content String
  * on a specific port
  * with methods to:
    * send forever in a loop at a given time period
    * send just one beacon
* Simple BeaconListener struct that can be setup to receive beacons:
  * matching a specific message contents
  * with a method that blocks sender and waits until a message is received
    * with an optional timeout value to wait for or None to wait indefinitely
* Simple 'announce' and 'listen' binaries that use the library as examples
* A some Doc tests to keep the API docs correct
* Github Action to build then clippy check then test all

## 'announce' binary
Run this binary from the repo using `cargo run --bin announce` or just `announce` if you have installed the
crate with cargo.

It takes an optional command line parameter to specify the String for the beacon message to announce:
`cargo run --bin announce -- Hello`

## 'listen' binary
Run this binary from the repo using `cargo run --bin listen` or just `listen` if you have installed the
crate with cargo.

It takes an optional command line parameter to specify the String for the beacon message to wait for before exiting:
`cargo run --bin listen -- Hello`

# Notes on things to add in the future
Server
- sending on background thread
  (look at implementing via async and not need to have a thread per beacon, just like a timer?)
  - start sending multiple beacons in parallel
- stop sending a specific beacon
- stop all beacons with one call?
- include an arbitrary Byte sequence to also send as meta-data
    client must serialize so we don't need to include a serialization lib or format
  
Listener
  - a protocol field?
- to parse any meta-data it must know about the service and know how to parse the format of it's meta-data

Call to wait for a beacon and then run a supplied closure when it is found?

- supply an optional regex to match the beacon name  
  https://crates.io/crates/regex
  to filter responses.
Make that feature and the crate inclusion behind a feature to keep the 
project as small as possible.