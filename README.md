[![crates.io](https://img.shields.io/crates/v/tcx.svg)](https://crates.io/crates/tcx)

# tcx

Training Center XML (TCX) parser written in Rust. TCX is an XML-based file format that is used for exchanging fitness tracking information from runs, bike rides, etc. It builds on Rust's serde deserialization framework.

## Example

```rust
use tcx;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let result = crate::tcx::read_file("tests/20210119_run_garmin_fenix6.tcx");
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

- 0.9.3 - Exposed all enums and structs with documentation. Added some traits to make life easier.
- 0.9.2 - Changed the return type so that the error is being passed back to the caller.
- 0.9.1 - Added ability to read speed and power data from the extensions field, also flushed out more structures from the specification.
- 0.9.0 - Initial release. Successfully parsed a few example files from Garmin Connect, but does not yet implement the entire specification.

## License

This project is licensed under the [MIT license](./LICENSE).
