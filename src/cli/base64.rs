use std::{fmt, str::FromStr};

use super::verify_input_file;
use clap::Parser;

/// Base64 子命令
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64DecodeOpts),
}

/// Base64 encode 命令参数
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value="-",help = "Input file path")]
    pub input: String,

    #[arg(long,value_parser= parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

/// Base64 decode 命令参数
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value="-",help = "Input file path")]
    pub input: String,

    #[arg(long,value_parser= parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

/// Base64格式
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

/// 从字符串解析 Base64 编码格式
///
/// 实现 `FromStr` trait 后，可以通过 `.parse()` 方法将字符串转换为 `Base64Format` 枚举。
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // 转换为小写进行比较
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format: {}", s)),
        }
    }
}

/// 从枚举转换为字符串
///
/// 实现From<T> trait后，可以通过into()方法将Base64Format枚举转换为字符串
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

/// 格式化输出
///
/// 实现fmt::Display trait后，可以通过format!()方法将Base64Format枚举格式化为字符串
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
