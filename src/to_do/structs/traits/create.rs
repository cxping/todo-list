use serde_json::{Map, Value, json};

use crate::state::writer_to_file;

pub trait  Create{

    fn create(&self,title:&String,status:&String,state:&mut Map<String,Value>){
        state.insert(title.to_string(),json!(status));
        writer_to_file("./state.json", state);
        println!("\n\n{}",title);
    }
}