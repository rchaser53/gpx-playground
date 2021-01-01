use std::fs::File;
use std::io::BufReader;

use xml::reader::EventReader;
use xml::writer::EmitterConfig;

mod cli;
mod parse;
mod trkseg;

use cli::{get_option, Opt};
use parse::parse;

fn main() {
    let Opt { input, output } = get_option();

    let file = BufReader::new(File::open(input).unwrap());
    let parser = EventReader::new(file);

    let mut file = File::create(output).unwrap();
    let writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut file);

    parse(parser, writer).unwrap();
}
