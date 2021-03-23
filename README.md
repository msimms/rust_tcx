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
    let _result = crate::tcx::read(&mut reader);
}
```

## Current Status
This is a work-in-progress.

## License
This project is licensed under the [MIT license](./LICENSE).
