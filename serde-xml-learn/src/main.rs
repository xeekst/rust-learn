use std::fs;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Configuration")]
struct Configuration {
    #[serde(rename = "CheckPoint")]
    check_point: i32,
    #[serde(rename = "ITCode")]
    itcode: String,
    #[serde(rename = "TestPlanName")]
    testplan_name: String,
    #[serde(rename = "Case", deserialize_with = "deserialize_list")]
    cases: Vec<Case>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Case {
    #[serde(rename = "CaseID")]
    case_id: i32,
    #[serde(rename = "CaseName")]
    case_name: String,
    #[serde(rename = "Order")]
    order: i32,
    #[serde(rename = "Filter")]
    filter: String,
    #[serde(rename = "PlanTime")]
    plan_times: i32,
    #[serde(rename = "ActualTime")]
    actual_times: i32,
    #[serde(rename = "Reboot")]
    reboot: bool,
}

fn main() {
    let s = fs::read_to_string("default.xml").unwrap();
    let r: Configuration = from_str(s.as_str()).unwrap();
    println!("{:?}", r);
    println!("Hello, world!");
}
