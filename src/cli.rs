use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum Command {
    Reverse {
        input: PathBuf,
        output: PathBuf,
    },
    Concat {
        input_a: PathBuf,
        input_b: PathBuf,
        output: PathBuf,
    },
}

pub fn get_command() -> Command {
    Command::from_args()
}
