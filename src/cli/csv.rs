use clap::Parser;
use std::{fmt, str::FromStr};

use super::verify_input_file;

/// 解析csv输出格式
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// csv 命令参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "Input file path")]
    pub input: String,

    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,

    #[arg(long, value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    pub header: bool,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
}

/// 解析输出格式
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

/// 将 OutputFormat 转换为 &'static str
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

/// 将 &str 转换为 OutputFormat
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}

/// 实现 fmt::Display
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 将 OutputFormat 转换为 &str
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
