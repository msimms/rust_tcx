# tcx
Training Center XML (TCX) parser written in Rust. It builds on Rust's serde deserialization framework.

## Example
```rust
extern crate tcx;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = File::open("tests/20210119_run_garmin_fenix6.tcx").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let result = crate::tcx::read(&mut reader);
    let activities = result.activities.unwrap();

    for activity in activities.activities {
        for lap in activity.laps {
            for track in lap.tracks {
                for trackpoint in track.trackpoints {
                    ...
                }
            }
        }
    }
}
```

## Current Status
It is currently able to parse several example TCX files, specifically ones for running and cycling activities, but does not implement the entire specification.

## Revision History
* 0.9.2 - Changed the return type so that the error is being passed back to the caller.
* 0.9.1 - Added ability to read speed and power data from the extensions field, also flushed out more structures from the specification.
* 0.9.0 - Initial release. Successfully parsed a few example files from Garmin Connect, but does not yet implement the entire specification.

## License
This project is licensed under the [MIT license](./LICENSE).
