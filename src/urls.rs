use std::{error::Error, path::Path, sync::Arc};

use tokio::{fs::File, io::AsyncReadExt};

pub async fn read_from_file(path: &str) -> Result<Vec<Arc<String>>, Box<dyn Error>> {
    let mut url = Vec::new();
    let path = Path::new(path);

    let mut file = File::open(path).await?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).await?;

    for line in buffer.lines() {
        url.push(Arc::new(line.to_string()));
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::read_from_file;

    #[tokio::test]
    async fn test_read_from_file() {
        let urls = read_from_file("testdata/sr.txt").await.unwrap();
        print!("{:?}", urls)
    }
}
