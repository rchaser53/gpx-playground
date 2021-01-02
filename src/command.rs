use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, Result};
use xml::writer::EmitterConfig;

use super::cli::Command;
use super::parse::reverse;

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
    }
}
