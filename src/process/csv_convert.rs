use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/*
把一个 CSV 文件的内容读取出来，并转换成 JSON 格式保存到另一个文件中
 */
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 读取CSV文件
    let mut reader = Reader::from_path(input)?;
    // 创建一个用于存储数据的向量
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();
    // 反序列化CSV文件
    for result in reader.records() {
        // 处理每一行数据
        let record = result?;

        // headers.iter() -> 使用headers的迭代器
        // record.iter() -> 使用record的迭代器
        // zip() -> 将两个迭代器合并为一个元祖的迭代器[(header. record),..]
        // collect::<Value>() -> 将元祖转换为JSON对象
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}
