use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// target: rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let opts: Opts = Opts::parse();
    // 处理子命令
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(&csv_opts.input, &csv_opts.output)?;
        }
    }
    // 返回结果
    Ok(())
}
