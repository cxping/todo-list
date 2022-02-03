
use actix_web::{App, HttpRequest, HttpServer, Responder};
mod processes;
mod state;
mod to_do;
mod middleware;
mod views;

// use to_do::structs::done::Done;
// use to_do::structs::pending::Pending;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let command: &String = &args[1];
//     let title: &String = &args[2];
//     let state: Map<String, Value> = read_file("./state.json");
//     let status: String;
//     match &state.get(*&title) {
//         Some(result) => {
//             status = result.to_string().replace('\"', "");
//         }
//         None => {
//             status = String::from("pending");
//         }
//     }
//     let item = to_do_factory(&status, title).expect(&status);
//     processes_input(item, command.to_string(), &state);
// }
#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(|| {
        App::new().configure(views::views_factory)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(3)
    .run()
    .await
}
#[warn(dead_code)]
async fn greet(req : HttpRequest)->impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",name)

}
