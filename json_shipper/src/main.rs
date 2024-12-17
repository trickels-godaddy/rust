use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct OsInfo {
    operating_system: String,
}

fn main() {
    let os_name = env::consts::OS.to_string();
    let os_info = OsInfo { operating_system: os_name };
    let json_output = serde_json::to_string(&os_info).unwrap();
    println!("{}", json_output);
}
