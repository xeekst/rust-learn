use std::{fs, path::PathBuf, env};

use quick_xml::{de::Deserializer, DeError};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]  
struct Configuration {
    #[serde(rename="$unflatten=CheckPoint")]
    check_point: i32,
    #[serde(rename="$unflatten=ITCode")]
    itcode: String,
    #[serde(rename="$unflatten=TestPlanName")]
    testplan_name: String,
    #[serde(rename="$unflatten=Case")]
    cases: Vec<Case>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Case{
    #[serde(rename="$unflatten=CaseID")]
    case_id: i32,
    #[serde(rename="$unflatten=CaseName")]
    case_name: String,
    #[serde(rename="Order")]
    order : i32,
    #[serde(rename="$unflatten=Filter")]
    filter: String,
    #[serde(rename="$unflatten=PlanTime")]
    plan_times: i32,
    #[serde(rename="$unflatten=ActualTime")]
    actual_times: i32,
    #[serde(rename="$unflatten=Reboot")]
    reboot: bool
}

fn from_str<'de, T>(s: &'de str) -> Result<T, DeError>
where
    T: Deserialize<'de>,
{
    // Log XML that we try to deserialize to see it in the failed tests output
    dbg!(s);
    let mut de = Deserializer::from_str(s);
    let result = T::deserialize(&mut de);

    // If type was deserialized, the whole XML document should be consumed, so continue consume, it will be UnexpectedEof
    if let Ok(_) = result {
        match <()>::deserialize(&mut de) {
            Err(DeError::UnexpectedEof) => {
                println!("1");

                ()
            },
            e => {
                println!("2");

                panic!("Expected end `UnexpectedEof`, but got {:?}", e)
            },
        }
    }

    result
}

fn main() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    
    println!("current dir: {:?}",  env::current_dir());
    println!("current dir: {:?}",  env::current_exe().unwrap().parent());
    let s = fs::read_to_string("default.xml").unwrap();
    let r:Configuration = from_str(&s).unwrap();
    println!("{:?}",r);

    // let mut reader = Reader::from_file("default.xml").unwrap();
    // reader.trim_text(true);
    // let mut buf = Vec::new();
    // let mut d = Deserializer::from_reader(reader);
    // let r = Configuration::deserialize(&mut d);
    // let mut count = 0;
    // let mut buf = Vec::new();
    // //let txt = Vec::new();
    // loop {
    //     match reader.read_event_into(&mut buf).unwrap() {
    //         Event::Start(e) => println!("e:{:?}", e),
    //         Event::Text(e) => println!("e:{:?}", e),
    //         Event::Eof => break,
    //         _ => (),
    //     }
    // }
    // println!("{:?}", txt);
}
