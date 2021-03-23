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

use serde_derive::{Deserialize};
use std::io::Read;
use std::io::BufReader;
use chrono::{DateTime, Utc};

extern crate chrono;
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default)]
pub struct HeartRate {
    #[serde(rename="Value")]
    pub value: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Position {
    #[serde(rename="LatitudeDegrees")]
    pub latitude: f64,
    #[serde(rename="LongitudeDegrees")]
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
pub struct Trackpoint {
    #[serde(rename="Time")]
    pub time: DateTime<Utc>,
    #[serde(rename="Position")]
    pub position: Position,
    #[serde(rename="AltitudeMeters")]
    pub altitude_meters: f64,
    #[serde(rename="DistanceMeters")]
    pub distance_meters: f64,
    #[serde(rename="HeartRateBpm")]
    pub heart_rate: Option<HeartRate>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Track {
    #[serde(rename="Trackpoint")]
    pub trackpoints: Vec<Trackpoint>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Lap {
    #[serde(rename="TotalTimeSeconds")]
    pub total_time_seconds: f64,
    #[serde(rename="DistanceMeters")]
    pub distance_meters: f64,
    #[serde(rename="MaximumSpeed")]
    pub maximum_speed: Option<f64>,
    #[serde(rename="Calories")]
    pub calories: Option<f64>,
    #[serde(rename="AverageHeartRate")]
    pub average_heart_rate: Option<f64>,
    #[serde(rename="MaximumHeartRate")]
    pub maximum_heart_rate: Option<f64>,
    #[serde(rename="Track")]
    pub tracks: Vec<Track>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Activity {
    #[serde(rename="Sport")]
    pub sport: String,
    #[serde(rename="Id")]
    pub id: String,
    #[serde(rename="Lap")]
    pub laps: Vec<Lap>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Activities {
    #[serde(rename="Activity")]
    pub activities: Vec<Activity>,
}

#[derive(Deserialize, Debug, Default)]
pub struct History {
}

#[derive(Deserialize, Debug, Default)]
pub struct Workouts {
}

#[derive(Deserialize, Debug, Default)]
pub struct Courses {
}

#[derive(Deserialize, Debug, Default)]
pub struct Extensions {
}

#[derive(Deserialize, Debug, Default)]
pub struct Folders {
    #[serde(rename="History")]
    pub history: Option<History>,
    #[serde(rename="Workouts")]
    pub workouts: Option<Workouts>,
    #[serde(rename="Courses")]
    pub courses: Option<Courses>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TrainingCenterDatabase {
    #[serde(rename="Activities")]
    pub activities: Option<Activities>,
    #[serde(rename="Folders")]
    pub folders: Option<Folders>,
    #[serde(rename="Courses")]
    pub courses: Option<Courses>,
    #[serde(rename="Extensions")]
    pub extensions: Option<Extensions>,
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> TrainingCenterDatabase {
    let tcx: TrainingCenterDatabase = serde_xml_rs::from_reader(reader).unwrap();
    tcx
}
