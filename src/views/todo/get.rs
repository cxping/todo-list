use actix_web::Responder;

use serde_json::{Value, Map};
use crate::{state::read_file, to_do::{to_do_factory, ItemTypes}};
use  crate::to_do::structs::json_serialization::to_do_items;

pub async fn get()->impl Responder{
    let  state :Map<String,Value> = read_file(&"./state.json");
    let mut arry_buffer = Vec::new();
    for (_key,value) in state  {
        let item_type: String = String::from(
            value.as_str().unwrap());
        let item: ItemTypes =to_do_factory (&item_type,&_key.as_str()).unwrap();
        arry_buffer.push(item);
    }
    let return_package: to_do_items::ToDoItems =to_do_items::ToDoItems::new(arry_buffer);
    return return_package;
}