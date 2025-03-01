use std::str;

pub fn parse_cves(path: &str) -> Vec<String> {
    Vec::new()

    // add business logic
}

#[test]
fn parse_cves_returns_high_serverities_cves_from_sarif_file() {
    let cves = parse_cves("tests/data/debian.sarif.json");
    assert_eq!(cves, vec!["CVE-2024-45338"]);
}
