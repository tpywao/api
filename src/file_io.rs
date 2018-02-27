use std::fs::OpenOptions;
use std::io::{
    BufWriter,
    Write,
    Result
};

pub fn write_file(fname: &str, data: &str) -> Result<()> {
    let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(fname)?;
    let mut f = BufWriter::new(file);
    f.write_all(data.as_bytes())
}
