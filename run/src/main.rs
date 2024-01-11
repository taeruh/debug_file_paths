use ignore::WalkBuilder;

fn main() {
    let path = std::env::args().nth(1).unwrap_or("./".to_string());
    for results in WalkBuilder::new(path).hidden(false).build() {
        match results {
            Ok(entry) => {
                println!("SHOW: {}", entry.path().display());
            },
            Err(err) => {
                println!("ERROR: {}", err);
            },
        }
    }
}
