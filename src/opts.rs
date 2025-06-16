use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "Input file path")]
    pub input: String,

    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    pub output: String,

    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    pub header: bool,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename.is_empty() {
        return Err("Input file path can not be empty");
    }
    if !Path::new(filename).exists() {
        return Err("Input file does not exist");
    }
    Ok(filename.to_string())
}
