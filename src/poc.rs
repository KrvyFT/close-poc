use std::{error::Error, sync::Arc};

use reqwest::{header::HOST, Response as ReqwestResponse};
use tokio::{
    task::{self, JoinSet},
    time::Duration,
};

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE,
    USER_AGENT,
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

impl Poc {
    pub fn from_json(path: &str) -> Result<Poc, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let poc = serde_json::from_reader(file)?;
        Ok(poc)
    }

    pub fn to_json(&self) -> Result<String, std::io::Error> {
        Ok(serde_json::to_string(&self)?)
    }

    pub async fn req_get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let res = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .build()?;
        let full_url = format!("http://{}{}", url, self.requests.payload);
        let resp = res
            .get(full_url)
            .header(HOST, url)
            .headers(self.requests.headers.to_maps())
            .timeout(Duration::from_secs(3))
            .send()
            .await?;

        if !self.is_exist(resp).await {
            return Ok(format!("EXIST: {}", url));
        }

        Ok(format!("Finished: {}", url))
    }

    pub async fn req_post(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let res = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .build()?;
        let full_url = format!("http://{}{}", url, self.requests.payload);
        let resp = res
            .post(full_url)
            .header(HOST, url)
            .headers(self.requests.headers.to_maps())
            .body(self.requests.data.clone())
            .timeout(Duration::from_secs(3))
            .send()
            .await?;

        if !self.is_exist(resp).await {
            return Ok(format!("EXIST: {}", url));
        }

        Ok(format!("Finished: {}", url))
    }

    async fn is_exist(&self, resp: ReqwestResponse) -> bool {
        if resp.status() == self.response.status_code
            && resp.text().await.unwrap().find(&self.response.text) != None
        {
            false
        } else {
            true
        }
    }

    pub async fn check_vulnerabilitie(&self, url: Arc<String>) -> Result<String, Box<dyn Error>> {
        let result = match self.requests.method {
            crate::poc::Method::GET => self.req_get(&url).await?,
            crate::poc::Method::POST => self.req_post(&url).await?,
        };
        Ok(result)
    }

    pub fn check_all_vulnerabilities(
        self,
        urls: Vec<Arc<String>>,
    ) -> JoinSet<Result<(), task::JoinError>> {
        let mut join_set = JoinSet::new();
        let arcs = Arc::new(self.clone());
        for url in urls {
            let handle_self = arcs.clone();
            join_set.spawn(task::spawn(async move {
                match handle_self.check_vulnerabilitie(url.clone()).await {
                    Ok(s) => println!("{s}"),
                    Err(e) => print!("{:#?}\n", e),
                }
            }));
        }

        join_set
    }
}
