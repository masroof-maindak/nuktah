use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = File::open("src.nkt")?;
    let mut src_code = String::new();

    f.read_to_string(&mut src_code)?;
    print!("{src_code}");
    Ok(())
}
