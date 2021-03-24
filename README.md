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
This is a work-in-progress.

## Revision History
0.9.0 - Initial release. Successfully parsed a few example files from Garmin Connect, but does not yet implement the entire specification.

## License
This project is licensed under the [MIT license](./LICENSE).
