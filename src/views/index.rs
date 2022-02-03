use actix_web::{HttpRequest, web};

pub(crate) async fn index(info: HttpRequest) -> String {
    format!("Welcome {:?}!", info)
}

pub fn index_factory(app:&mut web::ServiceConfig){
    app.route("/", web::get().to(index)).route("/index", web::get().to(index));
}