use crate::processes::processes_input;
use crate::state::read_file;
use crate::to_do::structs::json_serialization::to_do_items::{self, ToDoItem};
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};
pub async fn edit(to_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file(&"./state.json");
    let item_title: &String = &to_item.title.clone();
    let status: String;
    match &state.get(item_title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => return HttpResponse::NotFound().json(format!("{} not in state", item_title)),
    }
   
    if &status == &to_item.status {
        println!("{}",status);
        println!("{}",&to_item.status);
        return HttpResponse::Ok().json(super::result_state());
    }
    match  to_do_factory(&status,&item_title) {
        Ok(item) =>processes_input(item, String::from("edit"), &state),
        Err(_) => {
            return HttpResponse::BadRequest().json(format!("{} not accepted", status))
        },
    }
    return HttpResponse::Ok().json(super::result_state());
}
