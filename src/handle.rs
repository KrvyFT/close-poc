use std::{error::Error, sync::Arc};

use reqwest::{header::HOST, Response};
use tokio::{
    task::{self, JoinSet},
    time::Duration,
};

use crate::poc::Poc;

impl Poc {
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

    async fn is_exist(&self, resp: Response) -> bool {
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{poc::Pocs, urls::read_from_file};

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
