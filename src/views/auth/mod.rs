
use actix_web::web;

use super::path::Path;
use self::login::login;
use self::logout::logout;

pub mod login;
pub mod logout;

pub  fn auth_facory(app:&mut web::ServiceConfig){
    let base_path = Path{prefix:String::from("/auth")};
    app
    .route(base_path.define(&String::from("/login")).as_str(), web::get().to(login::login))
    .route(base_path.define(&String::from("/logout")).as_str(), web::get().to(logout::logout));
}