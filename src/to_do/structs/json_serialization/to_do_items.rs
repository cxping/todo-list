use std::io::Error;

use futures::future::{Ready, ready};
use serde::{Serialize, Deserialize};

use crate::to_do::{structs::base::Base, ItemTypes};
use actix_web::{web,Responder, HttpResponse};

#[derive(Serialize)]
pub struct  ToDoItems{
    pub done_item_count:i32,
    pub done_items:Vec<Base>,
    pub pending_item_count:i32,
    pub pending_items:Vec<Base>,
}
impl ToDoItems{
    pub fn new(input_items:Vec<ItemTypes>)->ToDoItems{
        let mut pedding_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items{
            match  item {
                ItemTypes::Pending(packed)=>{
                    pedding_array_buffer.push(
                        packed.super_struct);
                },
                ItemTypes::Done(packed)=>{
                    done_array_buffer.push(
                        packed.super_struct);
                }

            }
        }
        let done_count:i32 = done_array_buffer.len() as i32;
        let pending_count:i32 = pedding_array_buffer.len() as i32;
        return  ToDoItems{
            pending_items:pedding_array_buffer,
            pending_item_count:pending_count,
            done_items:done_array_buffer,
            done_item_count:done_count,
        };
    }
}

impl   Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
       let body = serde_json::to_string(&self).unwrap();
       ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

//如果需要在web：josn中接收，需要实现反序列化
#[derive(Debug,Serialize,Deserialize)]
pub struct  ToDoItem{
    pub title:String,
    pub status:String

}