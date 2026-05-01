use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("uso: bap-pack <input.app> <output.bap>");
        std::process::exit(2);
    }
    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);
    let payload = fs::read(&input).with_context(|| format!("no se pudo leer {:?}", input))?;

    let mut bap = Vec::new();
    bap.extend_from_slice(b"BAP1");
    bap.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bap.extend_from_slice(&payload);

    fs::write(&output, bap).with_context(|| format!("no se pudo escribir {:?}", output))?;
    println!("empaquetado {:?} -> {:?}", input, output);
    Ok(())
}
