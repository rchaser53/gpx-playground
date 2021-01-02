mod cli;
mod command;
mod parse;
mod trkseg;

use cli::get_option;
use command::execute_command;

fn main() {
    execute_command(get_option()).unwrap();
}
