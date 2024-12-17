use std::fs;
use serde_sarif::sarif::Sarif;

fn main() {    
    let data = fs::read_to_string("tests/data/sample.sarif").unwrap();
    let sarif: Sarif = serde_json::from_str(&data).unwrap();
    
    for run in sarif.runs  {
        for result in run.results.unwrap() {
            println!("{}, {}", result.level.unwrap(), result.message.text.unwrap())
        }
    }
}