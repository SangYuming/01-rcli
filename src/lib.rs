mod opts;
mod process;

pub use opts::{GenPassOpts, Opts, SubCommand}; // 将Opts,SubCommand提升为顶层API
pub use process::{process_csv, process_genpass};
