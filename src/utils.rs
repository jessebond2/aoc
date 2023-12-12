use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn read_lines_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|l| l.expect("can't parse")).collect())
}

pub fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read string");

    Ok(data)
}
