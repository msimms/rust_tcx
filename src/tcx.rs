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

#[derive(Default)]
pub struct Trackpoint {
    pub time: u64,
}

#[derive(Default)]
pub struct Track {
    pub trackpoints: Vec<Trackpoint>,
}

#[derive(Default)]
pub struct Lap {
    pub total_time_seconds: f64,
    pub distance_meters: f64,
    pub maximum_speed: f64,
    pub calories: f64,
    pub average_heart_rate: f64,
    pub maximum_heart_rate: f64,
    pub tracks: Vec<Track>,
}

#[derive(Default)]
pub struct Activity {
    pub sport: String,
    pub id: String,
    pub laps: Vec<Lap>,
}

#[derive(Default)]
pub struct Tcx {
    pub activities: Vec<Activity>,
}
