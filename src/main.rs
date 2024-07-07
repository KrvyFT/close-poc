use poc::Pocs;
use urls::read_from_file;

mod handle;
mod poc;
mod urls;

#[tokio::main(worker_threads = 128)]
async fn main() {
    let pocs = Pocs::from_json("testdata/get.json").unwrap();
    let urls = read_from_file("testdata/sr.txt").await.unwrap();

    let poc = pocs.0.get(0).unwrap();
    let mut join_set = poc.clone().check_all_vulnerabilities(urls);
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(_) => continue,
            Err(e) => println!("Task panicked: {:?}", e),
        }
    }
}
