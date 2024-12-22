use std::{
    env,
    error::Error,
    fs
};

use serde_sarif::sarif::Sarif;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:");
        println!("screener: <sarif-report.json>");
        return Ok(());
    }

    let data = fs::read_to_string(&args[1]).unwrap();
    let sarif: Sarif = serde_json::from_str(&data).unwrap();
    
    for run in sarif.runs  {
        for result in run.results.unwrap() {
            println!("{}, {}", result.level.unwrap(), result.message.text.unwrap())
        }
    }

    Ok(())
}