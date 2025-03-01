use std::{
    collections::HashMap,
    env::{self, args},
    error::Error,
    fs,
};

use serde_sarif::sarif::Sarif;

use anyhow::{Ok, Result};

use screener::parse_cves;

fn main() -> Result<()> {
    let path = args().skip(1).next().unwrap();
    let cves = parse_cves(&path);
    for cve in cves {
        println!("{cve}")
    }
    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         println!("Usage:");
//         println!("screener: <sarif-report.json>");
//         return Ok(());
//     }

//     let data = fs::read_to_string(&args[1]).unwrap();
//     let sarif: Sarif = serde_json::from_str(&data).unwrap();

//     for run in sarif.runs {
//         let mut cve = HashMap::new();

//         for rule in run.tool.driver.rules.unwrap() {
//             cve.insert(
//                 rule.id,
//                 rule.properties
//                     .unwrap()
//                     .additional_properties
//                     .get("cvssV3_severity")
//                     .unwrap()
//                     .as_str()
//                     .unwrap()
//                     .to_string(),
//             );
//         }

//         //println!("{cve:?}");

//         // for r in run.results.unwrap().iter().filter(|r|{
//         //     let id = &r.rule_id.clone().unwrap();
//         //     let level = cve.get(id).unwrap();

//         //     level == "HIGH"
//         // }) {
//         //     println!("CVE: {:?}", r.rule_id.as_ref().unwrap());
//         // };

//         // implement filter map here:
//         for id in run.results.unwrap().into_iter().filter_map (|r|{
//             let id = &r.rule_id.clone().unwrap();
//             let level = cve.get(id).unwrap();

//             if level == "HIGH" {
//                 Some(id.clone())
//             } else {
//                 None
//             }
//         }) {
//             println!("CVE: {:?}", id);
//         };

//         // for r in run.results.unwrap() {
//         //     let id = r.rule_id.unwrap();
//         //     let level = cve.get(&id).unwrap();

//         //     if level == "HIGH" {
//         //         println!("Level: {}, CVE: {}", level, &id);
//         //     }

//         //     // if r.level.unwrap().as_str().unwrap() == "HIGH" {
//         //     //     println!("HIGH CVE: {}", r.message.text.unwrap())
//         //     // }

//         // }
//     }

//     Ok(())
// }
