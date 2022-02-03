pub mod auth;
pub mod path;
pub mod todo;
pub mod index;
pub mod token;
pub mod app;
use actix_web::web;

pub fn views_factory(app:&mut web::ServiceConfig){
    // index::index_factory(app);
    auth::auth_facory(app);
    todo::item_factory(app);
    app::app_factory(app);
}