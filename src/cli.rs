use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum Command {
    Reverse { input: PathBuf, output: PathBuf },
}

pub fn get_command() -> Command {
    Command::from_args()
}
