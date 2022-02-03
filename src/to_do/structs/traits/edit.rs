use serde_json::{Map, Value, json};
use crate::state::writer_to_file;
pub trait  Edit {
    fn set_to_done(&self,title:&String,state:&mut Map<String,Value>){
        state.insert(title.to_string(),json!(String::from("done")));
        writer_to_file("./state.json",state);
        println!("{} is being set to done", title);
    }
    fn set_to_pending(&self,title:&String,state:&mut Map<String,Value>){
        state.insert(title.to_string(),json!(String::from("pending")));
        writer_to_file("./state.json",state);
        println!("{} is being set to pending", title);
    }
}