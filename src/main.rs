use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// target: rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let opts: Opts = Opts::parse();
    // 处理子命令
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    // 返回结果
    Ok(())
}
