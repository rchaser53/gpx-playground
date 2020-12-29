extern crate gpx;

use std::io::BufReader;
use std::fs::File;

use gpx::{read, write};
use gpx::{Gpx, Track, TrackSegment};

fn main() {
    let file = File::open("./test/wada.gpx").unwrap();
    let reader = BufReader::new(file);

    let mut gpx: Gpx = read(reader).unwrap();
    let mut track = &mut gpx.tracks[0];
    let mut segment = &mut track.segments[0];
    segment.points.reverse();
    let mut writer: Vec<u8> = Vec::new();
    let result = write(&gpx, std::io::stdout());
}