use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|l| l.expect("can't parse")).collect())
}
