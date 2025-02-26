use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use nuktah::compile_src;

fn main() -> std::io::Result<()> {
    let mut f = File::open("src.nkt")?;
    let mut src_code = String::new();

    let start = Instant::now();
    f.read_to_string(&mut src_code)?;
    let duration = start.elapsed();

    match compile_src(&mut src_code) {
        Ok(()) => {
            println!("Built in {} seconds.", duration.as_secs_f64());
            return Ok(());
        }

        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };
}
