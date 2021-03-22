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
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default)]
pub struct Trackpoint {
    #[serde(rename="Time")]
    pub time: u64,
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
pub struct TrainingCenterDatabase {
    #[serde(rename="Activity")]
    pub activities: Vec<Activity>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Tcx {
    #[serde(rename="TrainingCenterDatabase")]
    pub training_center_database: TrainingCenterDatabase,
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> Tcx {
    let tcx: Tcx = serde_xml_rs::from_reader(reader).unwrap();
    tcx
}
