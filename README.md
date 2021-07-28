# Simpdiscovery

Simpdiscovery is a simple rust crate to facilitate discovery of services on other machines (or other threads or
processes on the same machine) in the Local Area network (LAN) using UDP Broadcast datagrams or "beacons".

# Goals
* LAN wide detection of specific services by name and what IP they are at
* Simple to understand and use
* Small, few dependencies on the library and small memory and cpu footprint
* Simple beacon format that is easy to use and that doesn't introduce specific file format support into the library

# Non-Goals
* Discovery of services across LANs, WANs, the Web or in the cloud.
  
# Implemented so far
* Simple BeaconSender class that can be setup to send beacons on a specific port, with methods to
send forever in a loop or to send just one beacon.
* Simple BeaconListener class that can be setup to receive beacons on a specific port with a wait method 
  that blocks sender and waits until a message is received.
* Simple 'announce' and 'listen' binaries that use the library as examples
* Test to ensure sending and reception works
* Github Action to build then test all

# Next
* Allow the client to specify the message to send
* Specify sending frequency
* Filter for beacons received by name

# Implementation Notes
Simplest solution
Server
- sending on background thread
  (maybe implement via async and not need to have a thread per beacon, just like a timer?)
  - start sending other different ones
- stop sending a specific beacon
- able to send multiple beacons
- stop all beacons with one call?
- set sending frequency
- set name of the service (String)
- include an arbitrary Byte sequence to also send as meta-data
    client must serialize for me so we don't need to include a serialization lib or format
      
Set TTL of outgoing datagrams?


Listener
- can get all beacons and then handle them itself
  - minimal info is IP, and beacon name, meta-data is optional
  - port sent also?
  - a protocol field?
- can request beacons matching any of a set of String names and only get notified when is EQUALS, but gets full beacon found with the name
  - timestamp of when the beacon was sent / received?
- to parse any meta-data it must know about the service and know how to parse
the format of it's meta-data
  
Blocking call to wait for a matching beacon, with an optional timeout

Call to wait for a beacon and then run a supplied closure when it is found?

Set TTL for incoming datagrams and if not read in that time then discard them?


- want to avoid receiving own beacon? Could be from another thread in the same process and hence wanted?
- to support threads we would need some GUID? not just IP?

- additional: can supply an optional regex to match the beacon name  
  https://crates.io/crates/regex
  to filter responses.
Make that feature and the crate inclusion behind a feature to keep the 
project as small as possible.