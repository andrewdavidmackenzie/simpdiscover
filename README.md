
Goal
- LAN wide detection of specific services, what IP they are at and what 
services they offer
  
Principles
- small and dimple
- as few dependencies as possible
- not tied to any one file/beacon format

Simplest solution
Server
- include the crate and setup beacon sending. This will run on a background thread and return a beacon handle.
  (maybe implement via async and not need to have a thread per beacon, just like a timer?)
  - start sending other different ones
- stop sending a specific beacon using the handle
- able to send multiple beacons
- stop all beacons with one call
-- set sending frequency
-- set name of the servcice (String)
-- include an arbitrary Byte sequence to also send as meta-data
    client must serialize for me so we don't need to include a serialization lib or format
      
Set TTL of outgoing datagrams?


Listener
- can get all beacons and then handle them itself
  - minimal info is IP, and beacon name, meta-data is optional
  - port sent also?
  - a protocol field?
- can request beacons matching any of a set of String names and only get notified when is EQUALS, but get's full beacon found with the name
  - timestamp of when the beacon was sent / received?
- to parse any meta-data it must know about the service and know how to parse
the format of it's meta-data
  
Blocking call to wait for a matching beacon, with a timeout

Call to wait for a beacon and then run a supplied closure when it is found?

Set TTL for incoming datagrams and if not read in that time then discard them?


- want to avoid receiving own beacon? COuld be from another thread in the same process and hence wanted?
- to support threads we would need some GUID? not just IP?

- additional: can supply an optional regex to match the beacon name  
  https://crates.io/crates/regex
  to filter responses.

Binary
- server and listener binaries so they can be used from command line?


NOTES
##

Make that feature and the crate inclusion behind a feature to keep the 
project as small as possible.