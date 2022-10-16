use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ZDaily New",
    about = "Using for create new ZDaily with leetcode problem. Inherit HTTPS_PROXY."
)]
pub struct Opt {
    /// logging level by number of `v', default logging level is error,
    /// 1, 2, 3, 4 correspond to warn, info, debug, trace respectively
    #[structopt(short = "v", parse(from_occurrences))]
    pub log_level: u32,

    /// configuration file's, check example for more details
    #[structopt(short, long, default_value = "./content/", env = "ZDAILY_CONTENT_DIR", parse(from_os_str))]
    pub content_dir: PathBuf,
}
