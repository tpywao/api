use std::fs::OpenOptions;
use std::io::{
    BufWriter,
    Write,
    BufReader,
    Read,
    Result
};

pub fn read_file(fname: &str) -> Result<String> {
    let file = OpenOptions::new().read(true).open(fname)?;
    let mut f = BufReader::new(file);
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

pub fn write_file(fname: &str, data: &str) -> Result<()> {
    let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(fname)?;
    let mut f = BufWriter::new(file);
    f.write_all(data.as_bytes())
}
