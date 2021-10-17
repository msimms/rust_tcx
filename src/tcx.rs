// by Michael J. Simms
// Copyright (c) 2021 Michael J. Simms

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, modify, merge, publish, distribute, sublicense, and/or sell
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

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::io::{BufReader, Read};

use chrono;
use serde_json;
use serde_xml_rs;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Enums

/// Describes the intensity level for laps (`CourseLap` or `ActivityLap`) as either `Active` or `Resting`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Intensity {
    Active,
    Resting,
}

/// Describes how an event (for example, a lap) was triggered.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerMethod {
    Manual,
    Distance,
    Location,
    Time,
    HeartRate,
}

/// Describes the type of Course Point.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CoursePointType {
    Generic,
    Summit,
    Valley,
    Water,
    Food,
    Danger,
    Left,
    Right,
    Straight,
    FirstAid,
    FourthCategory,
    ThirdCategory,
    SecondCategory,
    FirstCategory,
    HorsCategory,
    Sprint,
}

/// The build type for the software that created the TCX file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuildType {
    Internal,
    Alpha,
    Beta,
    Release,
}

/// The type of speed indication used; either `Pace` (eg. minutes per km) or `Speed` (eg. meters per second).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpeedType {
    Pace,
    Speed,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Structs

/// Version information for the software that produced the TCX. Note: Does not follow [Semantic Versioning](https://semver.org).
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Version {
    /// Major version. Serializes to `VersionMajor`.
    #[serde(rename = "VersionMajor")]
    pub version_major: u16,

    /// Minor version. Serializes to `VersionMinor`.
    #[serde(rename = "VersionMinor")]
    pub version_minor: u16,

    /// Build major version. Serializes to `BuildMajor`.
    #[serde(rename = "BuildMajor")]
    pub build_major: Option<u16>,

    /// Build minor version. Serializes to `BuildMinor`.
    #[serde(rename = "BuildMinor")]
    pub build_minor: Option<u16>,
}

/// Empty placeholder for creator information in the `Course` struct.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AbstractSource {}

/// Empty placeholder for course name reference information in the `CourseFolder` struct.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NameKeyReference {}

/// Describes courses with extensions.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Courses {
    /// A folder of courses.
    #[serde(rename = "CourseFolder")]
    pub folder: Option<CourseFolder>,

    /// Additional extensional information about the courses.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Defines a folder for course information.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CourseFolder {
    /// Describes a self-contained folder. Serializes to `Folder.
    #[serde(rename = "Folder")]
    pub folder: Box<Option<CourseFolder>>,

    /// Describes any optional notes attached to the folder. Serializes to `Notes`.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    /// Optional name key reference for the course. Serializes to `CourseNameRef`.
    #[serde(rename = "CourseNameRef")]
    pub course_name_ref: Option<NameKeyReference>,

    /// Any extensional information about the folder. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Describes a course.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Course {
    /// Contains a lap within a course. Serializes to `CourseLap`.
    #[serde(rename = "CourseLap")]
    pub lap: Option<CourseLap>,

    /// Contains a list of tracks within the course. Serializes to `Track`.
    #[serde(rename = "Track")]
    pub tracks: Option<Vec<Track>>,

    /// Describes any optional notes attached to the folder. Serializes to `Notes`.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    /// Contains a (way-) point on a course. Serializes to `CoursePoint`.
    #[serde(rename = "CoursePoint")]
    pub course_point: Option<CoursePoint>,

    /// Identifies the creator for the course. Serializes to `Creator`.
    #[serde(rename = "Creator")]
    pub creator: Option<AbstractSource>,

    /// Any extensional information about the folder. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Describes a lap within a course.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CourseLap {
    /// Lap total time in seconds. Serializes to `TotalTimeSeconds`.
    #[serde(rename = "TotalTimeSeconds")]
    pub total_time_seconds: f64,

    /// Lap distance in meters. Serializes to `DistanceMeters`.
    #[serde(rename = "DistanceMeters")]
    pub distance_meters: f64,

    /// GPS position at the beginning of the lap. Serializes to `BeginPosition`.
    #[serde(rename = "BeginPosition")]
    pub begin_position: Option<Position>,

    /// Altitude in meters at the beginning of the lap. Serializes to `BeginAltitudeMeters`.
    #[serde(rename = "BeginAltitudeMeters")]
    pub begin_altitude_meters: Option<f64>,

    /// GPS position at the end of the lap. Serializes to `EndPosition`.
    #[serde(rename = "EndPosition")]
    pub end_position: Option<Position>,

    /// Altitude in meteres at the end of the lap. Serializes to `EndAltitudeMeters`
    #[serde(rename = "EndAltitudeMeters")]
    pub end_altitude_meters: f64,

    /// Average heart rate for the lap in Beats per Minute (BPM). Serializes to `AverageHeartRateBpm`.
    #[serde(rename = "AverageHeartRateBpm")]
    pub average_heart_rate: Option<f64>,

    /// Maximum heart rate for the lap in Beats per Minute (BPM). Serializes to `MaximumHeartRateBpm`
    #[serde(rename = "MaximumHeartRate")]
    pub maximum_heart_rate: Option<f64>,

    /// Intensity (`Active` or `Resting`) for this lap. Serializes to `Intensity`.
    #[serde(rename = "Intensity")]
    pub intensity: Option<Intensity>,

    /// Cadence (typically in Steps, Strokes or Revolutions per Minute) for the lap. Serializes to `Cadence`
    #[serde(rename = "Cadence")]
    pub cadence: Option<u8>,

    /// Optional extensional information about the lap. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Identifies a point of interest within a course.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoursePointName {
    pub token: u8,
}

/// Describes a point of interest within a course.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoursePoint {
    /// The name of the course point. Serializes to `Name`.
    #[serde(rename = "Name")]
    pub name: Option<CoursePointName>,

    /// The time the course point was recorded. Serializes to `Time`.
    #[serde(rename = "Time")]
    pub time: DateTime<Utc>,

    /// The GPS position of the course point. Serializes to `Position`.
    #[serde(rename = "Position")]
    pub position: Option<Position>,

    /// The altitude in meters for the course point. Serializes to `AltitudeMeters`.
    #[serde(rename = "AltitudeMeters")]
    pub altitude_meters: Option<f64>,

    /// The type of course point. Serializes to `PointType`.
    #[serde(rename = "PointType")]
    pub point_type: Option<CoursePointType>,

    /// Any additional notes that may have been recorded about the course point. Serializes to `Notes`.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    /// Optional extensional information about the course point. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Contains heart rate information.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HeartRate {
    /// Heart rate value. Used by both Average and Maxmimum heart rate indications in various places. Serializes to `Value`.
    #[serde(rename = "Value")]
    pub value: f64,
}

/// GPS position in degrees latitude and longitude.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Position {
    /// Degrees latitude. Positive numbers are North of the Equator, negative numbers are South. Serializes to `LatitudeDegrees`.
    #[serde(rename = "LatitudeDegrees")]
    pub latitude: f64,

    /// Degrees longitude. Positive numbers are East of the 0 meridian, negative numbers are West. Serializes to `LongitudeDegrees`.
    #[serde(rename = "LongitudeDegrees")]
    pub longitude: f64,
}

/// Describes an individual point in a Track.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trackpoint {
    /// Indicates the time the Trackpoint was recorded. Serializes to `Time`.
    #[serde(rename = "Time")]
    pub time: DateTime<Utc>,

    /// The GPS position at which the Trackpoint was recorded. Serializes to `Position`.
    #[serde(rename = "Position")]
    pub position: Option<Position>,

    /// The altitude in meters at the location where the Trackpoint was recorded. Serializes to `AltitudeMeters`.
    #[serde(rename = "AltitudeMeters")]
    pub altitude_meters: Option<f64>,

    /// The distance in meters covered when the track was first instantiated. Serializes to `DistanceMeters`.
    #[serde(rename = "DistanceMeters")]
    pub distance_meters: Option<f64>,

    /// Heart rate in Beats per Minute when the Trackpoint was recorded. Serializes to `HeartRateBtm`.
    #[serde(rename = "HeartRateBpm")]
    pub heart_rate: Option<HeartRate>,

    /// The cadence in Steps, Revolutions, or Strokes per Minute at the time when the Trackpoint was recorded. Serializes to `Candence`.
    #[serde(rename = "Cadence")]
    pub cadence: Option<u8>,

    /// Optional extensional information about the course point. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// Describes a Track as a list of Trackpoints.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Track {
    /// A list of Trackpoints that make up a Track. Serializes to `Trackpoint`.
    #[serde(rename = "Trackpoint")]
    pub trackpoints: Vec<Trackpoint>,
}

/// Contains summary information for each individual lap within an activity.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ActivityLap {
    /// Total lap duration in secons. Serializes to `TotalTimeSeconds`.
    #[serde(rename = "TotalTimeSeconds")]
    pub total_time_seconds: f64,

    /// Total distance covered during the lap in meters. Serializes to `DistanceMeters`.
    #[serde(rename = "DistanceMeters")]
    pub distance_meters: f64,

    /// Maximum speed in Meters/Second obtained during the lap. Serializes to `MaximumSpeed`.
    #[serde(rename = "MaximumSpeed")]
    pub maximum_speed: Option<f64>,

    /// Number of calories burned during the lap. Serializes to `Calories`.
    #[serde(rename = "Calories")]
    pub calories: u16,

    /// Average heart rate in Beats per Minute (BPM) for the lap. Serializes to `AverageHeartRate`.
    #[serde(rename = "AverageHeartRate")]
    pub average_heart_rate: Option<f64>,

    /// Maximum heart rate in Beats per Minute (BPM) for the lap. Serializes to `MaximumHeartRate`.
    #[serde(rename = "MaximumHeartRate")]
    pub maximum_heart_rate: Option<f64>,

    /// Intensity level for the lap, either `Active` or `Resting`. Serializes to `Intensity`.
    #[serde(rename = "Intensity")]
    pub intensity: Option<Intensity>,

    /// Cadence (typically in Steps, Revolutions or Strokes per Minute) for the lap. Serializes to `Cadence`.
    #[serde(rename = "Cadence")]
    pub cadence: Option<u8>,

    /// Trigger method for the lap. Serializes to `TriggerMethod`.
    #[serde(rename = "TriggerMethod")]
    pub trigger_method: Option<TriggerMethod>,

    /// A list of tracks within the lap. Serializes to `Track`.
    #[serde(rename = "Track")]
    pub tracks: Vec<Track>,

    /// Any additional notes that may describe the lap. Serializes to `Notes`.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    /// Any extensional information about the lap. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

impl ActivityLap {
    /// Calculates the maximum and average heart rates based on the information recorded in the TrackPoints in each Track and sets the `average_heart_rate` and `maximum_heart_rate` fields.
    ///
    /// # Parameters
    ///
    /// None. `&mut self` is implicit and doesn't need to be called.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut tcx = crate::tcx::TrainingCenterDatabase::from_file("tests/20210119_run_garmin_fenix6.tcx").unwrap();
    /// tcx.activities.as_mut().unwrap().activities[0].laps[0].calc_heartrates();
    /// tcx.export_json("tests/20210119_run_garmin_fenix6.lap.json");
    /// ```
    pub fn calc_heartrates(&mut self) {
        let mut max_hr = 0.0;
        let mut avg_hr = 0.0;
        let mut num_tp = 0;
        for track in &self.tracks {
            num_tp += track.trackpoints.len();
            for point in &track.trackpoints {
                if let Some(hr) = &point.heart_rate {
                    avg_hr += hr.value; // Add up for avg heart rate
                    if hr.value > max_hr {
                        max_hr = hr.value; // Find max HR
                    }
                } // if let
            } // for point
        } // for track

        // Set max heart rata
        if max_hr > 0.0 {
            self.maximum_heart_rate = Some(max_hr);
        }

        // Set average heart rate
        if num_tp > 0 {
            self.average_heart_rate = Some(avg_hr / num_tp as f64);
        }
    } // pub fn
}

/// Holds high-level information about an activity. This includes a the name and (often) the start time for the activity, as well as a list of laps.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Activity {
    /// The name of the activity being performed. Serializes to `Sport`.
    #[serde(rename = "Sport")]
    pub sport: String,

    /// An identifier for the activity. This is often the start time of the activity. Serializes to `Id`.
    #[serde(rename = "Id")]
    pub id: String,

    /// A list of laps. Serializes to `Lap`.
    #[serde(rename = "Lap")]
    pub laps: Vec<ActivityLap>,

    /// An optional note or description of the activity. Serializes to `Notes`.
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

    /// Any extentional data about the activity. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

/// A list of the activities found in the TCX file
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Activities {
    #[serde(rename = "Activity")]
    pub activities: Vec<Activity>,
}

/// Placeholder struct for history information. Currently not used.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct History {}

/// Placeholder struct for workouts information. Currently not used.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Workouts {}

/// NS3 TPX Extension data.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ns3Tpx {
    /// Speed, typically in meters per second. Serializes to `Speed`.
    #[serde(rename = "Speed")]
    pub speed: Option<f64>,

    /// Excertion in Watts. Serializes to `Watts`.
    #[serde(rename = "Watts")]
    pub watts: Option<u16>,
}

/// Placeholder struct for extension data. Currently supports NS3 TPX extensions.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Extensions {
    /// NS3 type TPX extensions. Serializes to `TPX`.
    #[serde(rename = "TPX")]
    pub tpx: Option<Ns3Tpx>,
}

/// Folders for various types of information: History, Workouts and Courses.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Folders {
    /// Holds information about History data, which is currently an empty struct. Serializes to `History`.
    #[serde(rename = "History")]
    pub history: Option<History>,

    /// Holds information about workout data, which is currently an empty struct. Serializes to `Workouts`.
    #[serde(rename = "Workouts")]
    pub workouts: Option<Workouts>,

    /// Holds information about Course folders. Serializes to `Courses`.
    #[serde(rename = "Courses")]
    pub courses: Option<Courses>,
}

/// The top-level struct that contains all the information found in the TCX file, along with associated functions.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TrainingCenterDatabase {
    /// A list of activities, if there are any. Serializes to `Activities`.
    #[serde(rename = "Activities")]
    pub activities: Option<Activities>,

    /// Any folders that may be present in the file. Serializes to `Folders`.
    #[serde(rename = "Folders")]
    pub folders: Option<Folders>,

    /// Any courses that may be present in the file. Serializes to `Courses`.
    #[serde(rename = "Courses")]
    pub courses: Option<Courses>,

    /// Any extensions that may be present in the file. Serializes to `Extensions`.
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

impl TrainingCenterDatabase {
    /// Reads and parses TCX data from the filename specified.
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The name of the file to be read.
    ///
    /// # Returns
    ///
    /// `Result<Self, serde_xml_rs::Error>`
    ///
    /// # Example
    ///
    /// ```rust
    /// let my_activities = crate::tcx::TrainingCenterDatabase::from_file("tests/20210119_run_garmin_fenix6.tcx");
    /// ```
    ///
    /// # References
    ///
    /// [`serde_xml_rs::Error`](https://docs.rs/serde-xml-rs/0.5.1/serde_xml_rs/enum.Error.html)
    pub fn from_file(filename: &str) -> Result<Self, serde_xml_rs::Error> {
        let file = std::fs::File::open(filename).unwrap();
        let mut reader = std::io::BufReader::new(file);
        serde_xml_rs::from_reader(&mut reader)
    }

    /// Calculates heart rates for all ActivityLap items. For now.
    ///
    /// # Parameters:
    ///
    /// None. `&mut self` is implicit.
    ///
    /// # Returns:
    ///
    /// Nothing.
    ///
    /// # Example:
    ///
    /// ```rust
    /// let mut tcx = crate::tcx::TrainingCenterDatabase::from_file("tests/20210119_run_garmin_fenix6.tcx").unwrap();
    /// tcx.calc_heartrates();
    /// tcx.export_json("tests/20210119_run_garmin_fenix6.tcdb.json");
    /// ```
    pub fn calc_heartrates(&mut self) {
        // Calculate heart rate for ActivityLaps.
        // FIXME - There has to be a better way to do this. Using iterators gives all kinds of problems with borrowing.
        for act in 0..self.activities.as_ref().unwrap().activities.len() {
            for lap in 0..self.activities.as_ref().unwrap().activities[act].laps.len() {
                self.activities.as_mut().unwrap().activities[act].laps[lap].calc_heartrates();
            }
        }

        // TODO - Calculate heart rate for CourseLaps
    }

    /// Exports the parsed contents of the TCX file to JSON format
    ///
    /// # Parameters
    ///
    /// `filename: &str` -- The name of the JSON file to be produced.
    ///
    /// # Returns
    ///
    /// `Result<(), Box<dyn Error>>` -- Nothing if OK, Error if not.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut tcx = crate::tcx::TrainingCenterDatabase::from_file("tests/20210119_run_garmin_fenix6.tcx").unwrap();
    /// tcx.activities.as_mut().unwrap().activities[0].laps[0].calc_heartrates();
    /// tcx.export_json("tests/20210119_run_garmin_fenix6.json");
    /// ```
    pub fn export_json(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // Write the session data to JSON
        serde_json::to_writer_pretty(
            &std::fs::File::create(&std::path::PathBuf::from(filename))?,
            &self,
        )?;

        // Return safely
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions

/// Reads TCX data from a buffer previously defined.
///
/// # Parameters
///
/// `reader: &mut BufReader<R>` -- A buffer of a file previously opened.
///
/// # Returns
///
/// `Result<TrainingCenterDatabase, serde_xml_rs::Error>`
///
/// # Example
///
/// ```rust
/// let file = std::fs::File::open("tests/20210119_run_garmin_fenix6.tcx").unwrap();
/// let mut reader = std::io::BufReader::new(file);
/// let result = crate::tcx::read(&mut reader);
/// ```
///
/// # References
///
/// [`serde_xml_rs::Error`](https://docs.rs/serde-xml-rs/0.5.1/serde_xml_rs/enum.Error.html)
pub fn read<R: Read>(
    reader: &mut BufReader<R>,
) -> Result<TrainingCenterDatabase, serde_xml_rs::Error> {
    serde_xml_rs::from_reader(reader)
}

/// Reads TCX data from the filename specified.
///
/// # Parameters
///
/// `filename: &str` -- The name of the file to be read.
///
/// # Returns
///
/// `Result<TrainingCenterDatabase, serde_xml_rs::Error>`
///
/// # Example
///
/// ```rust
/// let my_activities = crate::tcx::read_file("tests/20210119_run_garmin_fenix6.tcx");
/// ```
///
/// # References
///
/// [`serde_xml_rs::Error`](https://docs.rs/serde-xml-rs/0.5.1/serde_xml_rs/enum.Error.html)
pub fn read_file(filename: &str) -> Result<TrainingCenterDatabase, serde_xml_rs::Error> {
    TrainingCenterDatabase::from_file(filename)
}
