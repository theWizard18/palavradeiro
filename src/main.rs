fn main() {
    match palavradeiro::run() {
        Ok(v)  => v.iter().for_each(|w| print!("{} ",w)),
        Err(e) => eprintln!("ERROR: {}", e),
    };
}
