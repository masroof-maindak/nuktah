use std::time::Instant;

use nuktah::compile_src;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: nktc <src.nkt>");
        std::process::exit(1);
    }

    let src_code = std::fs::read_to_string(&args[1])?;

    let start = Instant::now();
    let res = compile_src(&src_code);
    let duration = start.elapsed();

    if let Err(e) = res {
        eprintln!("{e:?}");
        std::process::exit(1);
    }

    println!("Built in {} seconds.", duration.as_secs_f64());
    Ok(())
}
