use anyhow::{Ok, Result};
use std::env::args;
use screener::parse_cves;

fn main() -> Result<()> {
    let path = args().skip(1).next().unwrap();
    let cves = parse_cves(&path)?;
    for cve in cves {
        println!("{cve}")
    }
    Ok(())
}
