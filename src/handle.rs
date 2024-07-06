use std::error::Error;

use crate::poc::Poc;

impl Poc {
    pub async fn check(self, url: &str) -> Result<(), Box<dyn Error>> {
        let res = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .build()?;
        let urll = url.to_owned() + &self.requests.payload;
        res.get(urll).json(&self.requests).send().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::poc::{self, Pocs};

    #[tokio::test]
    async fn test_check_vulnerabilities() -> Result<(), Box<dyn Error>> {
        let pocs = Pocs::from_json("src/poc.json").unwrap();
        for poc in pocs {
            poc.check("http://27.174.121.119:8001").await?;
        }

        Ok(())
    }
}
