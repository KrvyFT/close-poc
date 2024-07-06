use poc::Pocs;

mod handle;
mod poc;
mod urls;

fn main() {
    let pocs = Pocs::from_json("src/poc.json").unwrap();
    println!("{:#?}", pocs)
}
