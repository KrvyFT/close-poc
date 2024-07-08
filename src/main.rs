use app::{app, run};

mod app;
mod poc;
mod urls;

#[tokio::main(worker_threads = 128)]
async fn main() {
    let mut join_set = run(app()).await;
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(s)) => println!("{s}"),
            Ok(Err(e)) => println!("{:#?}", e),
            Err(e) => println!("Task panicked: {:?}", e),
        }
    }
}
