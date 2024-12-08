use std::fs::File;
use std::io::{self, BufRead,BufReader};
use std::path::Path; 

pub fn read_file_return_buffer<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buffered_reader = BufReader::new(file);
    buffered_reader.lines().collect()
}

