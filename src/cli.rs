use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    pub input: PathBuf,
    pub output: PathBuf,
}

pub fn get_option() -> Opt {
    Opt::from_args()
}
