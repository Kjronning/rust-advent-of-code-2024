use std::fs::File;
use std::io::BufReader;
use std::io::Read;


pub fn read(filename: &str, input: &mut String) {
    let file = File::open(filename).unwrap();
    BufReader::new(file).read_to_string(input);
}
