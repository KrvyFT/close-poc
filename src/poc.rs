use std::error::Error;

use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION,
    CONTENT_TYPE, USER_AGENT,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Poc {
    pub name: String,
    pub requests: Requests,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Requests {
    pub method: Method,
    pub payload: String,
    pub headers: Headers,
    #[serde(default)]
    pub data: String,
    pub dnslog: bool,
    pub sqltime: bool,
    pub filelocate: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Headers {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(default, rename = "Accept")]
    pub accept: String,
    #[serde(default, rename = "Accept-Encoding")]
    pub accept_encoding: String,
    #[serde(default, rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(default, rename = "Connection")]
    pub connection: String,
    #[serde(default, rename = "Content-Type")]
    pub content_type: String,
    #[serde(default, rename = "X-Requested-With")]
    pub x_requested_with: String,
}

impl Headers {
    pub fn to_maps(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(user_agent) = HeaderValue::from_str(&self.user_agent) {
            headers.insert(USER_AGENT, user_agent);
        }
        if let Ok(accept) = HeaderValue::from_str(&self.accept) {
            headers.insert(ACCEPT, accept);
        }
        if let Ok(accept_encoding) = HeaderValue::from_str(&self.accept_encoding) {
            headers.insert(ACCEPT_ENCODING, accept_encoding);
        }
        if let Ok(accept_language) = HeaderValue::from_str(&self.accept_language) {
            headers.insert(ACCEPT_LANGUAGE, accept_language);
        }
        if let Ok(connection) = HeaderValue::from_str(&self.connection) {
            headers.insert(CONNECTION, connection);
        }
        if let Ok(content_type) = HeaderValue::from_str(&self.content_type) {
            headers.insert(CONTENT_TYPE, content_type);
        }
        // if let Ok(x_requested_with) = HeaderValue::from_str(&self.x_requested_with) {
        // headers.insert(
        // .
        // x_requested_with,
        // );
        // }

        headers
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    pub status_code: u16,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pocs(pub Vec<Poc>);
impl Pocs {
    pub fn from_json(path: &str) -> Result<Pocs, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let poc = serde_json::from_reader(file)?;
        Ok(poc)
    }

    pub fn to_json(&self) -> Result<String, std::io::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl Iterator for Pocs {
    type Item = Poc;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
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
            let json_content =
                fs::read_to_string("testdata/get.json").expect("Unable to read poc.json");

            // 反序列化 JSON 内容到 Poc 结构体
            let poc: Pocs =
                serde_json::from_str(&json_content).expect("Failed to deserialize JSON to Poc");

            // 序列化 Poc 结构体到 JSON 字符串
            let serialized_poc =
                serde_json::to_string(&poc).expect("Failed to serialize Poc to JSON");

            // 反序列化回 Poc 结构体以验证
            let deserialized_poc: Pocs =
                serde_json::from_str(&serialized_poc).expect("Failed to deserialize JSON to Poc");
            println!("{:#?}", deserialized_poc);
            // 验证原始 Poc 和反序列化后的 Poc 是否相同
            assert_eq!(poc, deserialized_poc);
        }
    }
}
