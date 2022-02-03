use actix_web::web::{HttpRequest,HttpResponse};
use super::content_loader::read_file;
pub async fn items()->HttpResponse{
    let ites_html = read_file("./templates/main.html");
    HttpResponse::Ok().content_type("text/html;charset=utf-8")
    .body(ites_html)
}