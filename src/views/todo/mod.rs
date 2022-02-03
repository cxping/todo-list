mod create;
mod get;
mod edit;

use actix_web::{web};

use super::path::Path;

pub fn item_factory(app:&mut web::ServiceConfig){
    let base_path = Path{prefix:String::from("/item")};
   app.route(&base_path.define(&String::from("/create/{title}")), web::get().to(create::create));
   app.route(&base_path.define(&String::from("/get")), web::get().to(get::get));
   app.route(&base_path.define(&String::from("/edit")), web::put().to(edit::edit));
}


pub fn result_state()->String{
    r#"{"code":200,"msg":"修改成功","data":{}}"#.to_string()
}

#[warn(dead_code)]
pub fn error_state()->String{
    r#"{"code":201,"msg":"修改失败","data":{}}"#.to_string()
}