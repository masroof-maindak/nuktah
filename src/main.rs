use std::fs::File;
use std::io::prelude::*;

use nuktah::compile_src;

fn main() -> std::io::Result<()> {
    let mut f = File::open("src.nkt")?;
    let mut src_code = String::new();

    f.read_to_string(&mut src_code)?;

    match compile_src(&mut src_code) {
        Ok(()) => {
            println!("Built in {} seconds.", 10);
            return Ok(());
        }

        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };
}
