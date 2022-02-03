use serde_json::{json, Map, Value};
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json_result = serde_json::from_str(&data);
   
    if let Ok(val) = json_result {
        let  json: Value = val;
        let state: Map<String, Value> = json.as_object().unwrap().clone();
        return  state;
    }
    return Map::new();
}
pub fn writer_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to writer file")
}
