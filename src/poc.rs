use std::error::Error;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Poc {
    pub name: String,
    pub requests: Requests,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Requests {
    pub method: Method,
    pub payload: String,
    pub headers: Headers,
    pub dnslog: bool,
    pub sqltime: bool,
    pub filelocate: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Headers {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Response {
    pub status_code: u16,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
    GET,
    POST,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pocs(Vec<Poc>);

pub fn from_json(path: &str) -> Result<Pocs, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let poc = serde_json::from_reader(file)?;
    Ok(poc)
}

pub fn to_json(soc: Pocs) -> Result<String, std::io::Error> {
    Ok(serde_json::to_string(&soc)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_poc_serialization_and_deserialization() {
            // 读取 poc.json 文件
            let json_content = fs::read_to_string("src/poc.json").expect("Unable to read poc.json");

            // 反序列化 JSON 内容到 Poc 结构体
            let poc: Pocs =
                serde_json::from_str(&json_content).expect("Failed to deserialize JSON to Poc");

            // 序列化 Poc 结构体到 JSON 字符串
            let serialized_poc =
                serde_json::to_string(&poc).expect("Failed to serialize Poc to JSON");

            // 反序列化回 Poc 结构体以验证
            let deserialized_poc: Pocs =
                serde_json::from_str(&serialized_poc).expect("Failed to deserialize JSON to Poc");

            // 验证原始 Poc 和反序列化后的 Poc 是否相同
            assert_eq!(poc, deserialized_poc);
        }
    }
}
