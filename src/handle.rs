use std::error::Error;

use reqwest::header::{self, HeaderMap, USER_AGENT};

use crate::poc::Poc;

impl Poc {
    pub async fn req_get(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let res = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .build()?;
        let full_url = format!("{}{}", url, self.requests.payload);
        res.get(full_url)
            .header(USER_AGENT, self.requests.headers.user_agent.as_str())
            .send()
            .await?;

        Ok(())
    }

    pub async fn req_post(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let res = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .build()?;
        let full_url = format!("{}{}", url, self.requests.payload);
        let headers = HeaderMap::new();
        res.post(full_url)
            .headers(self.requests.headers.to_maps())
            .body(self.requests.data.clone())
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        poc::{self, Pocs},
        urls::{self, read_from_file},
    };

    #[tokio::test]
    async fn test_check_vulnerabilities() -> Result<(), Box<dyn Error>> {
        let pocs = Pocs::from_json("testdata/post.json").unwrap();
        let urls = read_from_file("testdata/ps.txt").await.unwrap();

        let poc = pocs.0.get(0).unwrap();
        for url in &urls {
            poc.req_post(&format!("http://{}", url)).await?;
        }

        Ok(())
    }
}
