use fuzzy::*;
use std::env;
use std::io;

/// Simple main entrypoint that assumes first argument is path to file to hash
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let digest = ssdeep::Hash::hash_from_file(path)?;
    println!("{}", digest);
    Ok(())
}
