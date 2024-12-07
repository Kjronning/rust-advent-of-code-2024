use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn read(filename: &str, input: &str) -> std::io::Lines<io::BufReader<File>>{
    read_file(filename).unwrap().read_to_string(&mut input);
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
#[inline]
fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}