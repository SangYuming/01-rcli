mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

/// 命令行参数
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 子命令
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// csv 命令
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),

    /// 生成密码命令参数
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    /// base64 命令
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

/// 验证输入文件是否存在
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}

/// 测试-验证输入文件是否存在
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("Input file does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file(""), Err("Input file does not exist"));
    }
}
