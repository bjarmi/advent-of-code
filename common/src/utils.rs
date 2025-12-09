use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Read};

/// Read input file and return lines as a vector of strings
pub fn read_input_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Read input file and return the entire contents as a string
pub fn read_input_string(file_path: &str) -> io::Result<String> {
    std::fs::read_to_string(file_path)
}

pub fn read_input_bytes(file_path: &str) -> Result<io::Bytes<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.bytes())
}

/// Read input file and return a buffered reader
pub fn read_input_buffered(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}
