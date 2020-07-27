use std::env;

use rlox3::repl;
use rlox3::InterpretError;

fn main() -> Result<(), InterpretError> {
    let mut args = env::args();
    args.next();

    let args: Vec<String> = args.collect();

    if args.len() > 1 {
        println!("Usage: rlox [file]");
        std::process::exit(64);
    } else if let Some(_filename) = args.first() {
        println!("TODO: Read file");
    } else {
        repl()?;
    }

    Ok(())
}
