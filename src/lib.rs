use std::{
    collections::HashMap, fs, str::{self}
};

use anyhow::{Ok, Result};

use serde_json;
use serde_sarif::sarif::Sarif;


pub fn parse_cves(path: &str) -> Result<Vec<String>> {
    let mut cves = Vec::new();

    // add business logic
    let data = fs::read_to_string(path).unwrap();
    let sarif: Sarif = serde_json::from_str(&data).unwrap();

    for run in sarif.runs {
        let mut cve = HashMap::new();

        for rule in run.tool.driver.rules.unwrap() {
            cve.insert(
                rule.id,
                rule.properties
                    .unwrap()
                    .additional_properties
                    .get("cvssV3_severity") // causes panic here - no key in the file
                    .unwrap_or(&serde_json::Value::String(String::from("HIGH")))
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
        }

        // implement filter map here:
        for id in run.results.unwrap().into_iter().filter_map (|r|{
            let id = &r.rule_id.clone().unwrap();
            let level = cve.get(id).unwrap();

            if level == "HIGH" {
                Some(id.clone())
            } else {
                None
            }
        }) {
            cves.push(id);
        };

    }

    Ok(cves)
}



#[test]
fn parse_cves_returns_high_serverities_cves_from_sarif_file() {
    let cves = parse_cves("tests/data/debian.sarif.json").unwrap();
    assert_eq!(cves, vec!["CVE-2024-45338"]);
}
