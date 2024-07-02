use poc::from_json;

mod poc;

fn main() {
    let pocs = from_json("src/poc.json").unwrap();
    println!("{:#?}", pocs)
}
