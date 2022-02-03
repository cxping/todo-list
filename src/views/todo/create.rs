use actix_web::HttpRequest;
use actix_web::Responder;
use serde_json::Map;
use serde_json::Value;

use crate::processes::processes_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;





pub async fn create(req:HttpRequest)->impl Responder{
    let state :Map<String,Value> = read_file(&String::from("./state.json"));
    let title:String = req.match_info().get("title").unwrap().to_string();
    let title_reference :String = title.clone();
    let item = to_do_factory(&String::from("pending"), &title).unwrap();
    processes_input(item, "create".to_string(), &state);
    return  format!("{} created",title_reference);
}