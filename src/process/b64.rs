use anyhow::Result;
use std::{fs::File, io::Read};

use base64::{engine::general_purpose::STANDARD, prelude::BASE64_URL_SAFE_NO_PAD, Engine as _};

use crate::Base64Format;

/// 编码
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("\n{}", encoded);

    Ok(())
}

/// 解码
pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let buf = buf
        .into_iter()
        .filter(|&b| b != b'\n' && b != b'\r' && b != b' ')
        .collect::<Vec<u8>>();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(&buf)?,
    };
    // TODO: decoded data might not be String (but for this example, we assume it is)
    let decoded_str = String::from_utf8(decoded)?;
    println!("\n{}", decoded_str);
    Ok(())
}

/**
 * 创建一个实现Read trait的Box指针，用于从标准输入或文件读取数据
 *
 * 参数:input: &str - 输入源路径，"-"表示标准输入，其他值表示文件路径
 * 返回:Result<Box<dyn Read>> - 成功时返回包含Read trait对象的Box，失败时返回错误
 */
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // 测试编码
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    // 测试解码
    #[test]
    fn test_process_decode() -> anyhow::Result<()> {
        // 创建一个临时文件并写入 Base64 编码数据
        let temp_file_path = "temp_base64.txt";
        let mut temp_file = File::create(temp_file_path)?;
        writeln!(temp_file, "aGVsbG8gd29ybGQhCg==")?; // 写入 Base64 编码的 "hello world!\n"

        let format = Base64Format::Standard;
        let result = process_decode(temp_file_path, format);
        assert!(result.is_ok());

        // 清理临时文件
        std::fs::remove_file(temp_file_path)?;
        Ok(())
    }
}
