use serde_json::{Map, Value};
use crate::state::writer_to_file;
pub trait  Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        writer_to_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
 }
}