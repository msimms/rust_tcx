// by Michael J. Simms
// Copyright (c) 2021 Michael J. Simms

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub use crate::tcx::read;

mod tcx;

#[cfg(test)]
mod tests {
    #[test]
    fn file1_run() {
        let file = std::fs::File::open("tests/20210119_run_garmin_fenix6.tcx").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let result = crate::tcx::read(&mut reader).unwrap();
        let activities = result.activities.unwrap();

        // Correct number of activities?
        assert_eq!(activities.activities.len(), 1);
        let activity = &activities.activities[0];

        // Correct number of laps?
        assert_eq!(activity.laps.len(), 1);
        let lap = &activity.laps[0];

        // Correct number of tracks?
        assert_eq!(lap.tracks.len(), 1);
        let track = &lap.tracks[0];

        // Correct number of trackpoints?
        assert_eq!(track.trackpoints.len(), 1232);
    }

    #[test]
    fn file2_ride_with_power() {
        let file = std::fs::File::open("tests/20210308_virtual_ride_with_power.tcx").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let result = crate::tcx::read(&mut reader).unwrap();
        let activities = result.activities.unwrap();

        // Correct number of activities?
        assert_eq!(activities.activities.len(), 1);
        let activity = &activities.activities[0];

        // Correct number of laps?
        assert_eq!(activity.laps.len(), 1);
        let lap = &activity.laps[0];

        // Correct number of tracks?
        assert_eq!(lap.tracks.len(), 1);
        let track = &lap.tracks[0];

        // Correct number of trackpoints?
        assert_eq!(track.trackpoints.len(), 1434);
        let first_trackpoint = &track.trackpoints[0];

        // Cadence?
        let cadence = first_trackpoint.cadence.unwrap();
        assert_eq!(cadence, 87);

        // Get the extension.
        let extensions = first_trackpoint.extensions.as_ref().unwrap();

        // Get the first power reading.
        let tpx = extensions.tpx.as_ref().unwrap();
        let watts = tpx.watts.unwrap();
        assert_eq!(watts, 216);
    }

    #[test]
    fn file3_yoga() {
        let file = std::fs::File::open("tests/20210323_yoga.tcx").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let result = crate::tcx::read(&mut reader).unwrap();
        let activities = result.activities.unwrap();

        // Correct number of activities?
        assert_eq!(activities.activities.len(), 1);
        let activity = &activities.activities[0];

        // Correct number of laps?
        assert_eq!(activity.laps.len(), 1);
        let _lap = &activity.laps[0];
    }
}
