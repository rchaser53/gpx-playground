use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub enum Opt {
    Reverse { input: PathBuf, output: PathBuf },
}

pub fn get_option() -> Opt {
    Opt::from_args()
}
