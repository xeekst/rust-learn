use std::fs::read_to_string;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
fn main() {
    let p = transtr("str");
    let p = transt::<Po>("str".to_owned());
    let p = trans("str".to_owned());
    println!("Hello, world!");
}

fn transtr(text: &str) -> impl Deserialize {
    let st = serde_json::from_str::<Po>(text).unwrap();

    st
}

fn transt<'de, T>(text: String) -> impl Deserialize<'de>
where
    T: Deserialize<'de>,
{
    let st = serde_json::from_str::<Po>(text.as_str()).unwrap();

    st
}

fn trans<'de>(text: String) -> impl Deserialize<'de> {
    let st = serde_json::from_str::<Po>(text.as_str()).unwrap();

    st
}

#[derive(Serialize, Deserialize, Debug)]
struct Po {
    name: String,
    value: Option<String>,
    s: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub listen: String,
}

pub fn get_app_config() -> AppConfig {
    let text = read_to_string("res/config/app.toml").unwrap();
    let toml = toml::from_str(text.as_str()).unwrap();

    toml
}

pub fn read_config<'a, T>(filename: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let tr = read_to_string(format!("res/config/{}", filename));
    match tr {
        Ok(text) => {
            println!("{}", text);
            let tt: T = toml::from_str(text.as_str()).unwrap();

            Ok(tt)
        }
        Err(e) => Err(e.to_string()),
    }
}
