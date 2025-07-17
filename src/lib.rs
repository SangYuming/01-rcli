mod cli;
mod process;

pub use cli::{Base64Format, Base64SubCommand, Opts, SubCommand}; // 将Opts,SubCommand提升为顶层API
pub use process::*;
