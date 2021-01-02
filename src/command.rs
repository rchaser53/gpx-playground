use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, Result};
use xml::writer::EmitterConfig;

use super::cli::Command;
use super::parse::{concat, reverse};

pub fn execute_command(command: Command) -> Result<()> {
    match command {
        Command::Reverse { input, output } => {
            let file = BufReader::new(File::open(input).unwrap());
            let parser = EventReader::new(file);

            let mut file = File::create(output).unwrap();
            let writer = EmitterConfig::new()
                .perform_indent(true)
                .create_writer(&mut file);
            reverse(parser, writer)
        }
        Command::Concat {
            input_a,
            input_b,
            output,
        } => {
            let file = BufReader::new(File::open(input_a).unwrap());
            let parser_a = EventReader::new(file);

            let file = BufReader::new(File::open(input_b).unwrap());
            let parser_b = EventReader::new(file);

            let mut file = File::create(output).unwrap();
            let writer = EmitterConfig::new()
                .perform_indent(true)
                .create_writer(&mut file);
            concat(parser_a, parser_b, writer)
        }
    }
}
