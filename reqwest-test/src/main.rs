fn main() {
    let text = reqwest::blocking::Client::builder().no_proxy().build().unwrap().get("http://127.0.0.1:4090/command/clean_preload_dir_and_update_mtsn?sub_task_id=1637764338859438082&except_name_like=aod.dat|BIOS&mac_path=O:\\dfcxact\\CDMENU\\mfgwinpe\\88672C65").send().unwrap().text().unwrap();
    println!("clean_preload_dir_and_update_mtsn:{text}");
}
