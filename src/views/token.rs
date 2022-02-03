


use actix_web::{web::{ServiceConfig}, dev::ServiceRequest};
fn check_password(password:String)->Result<String,&'static str>{
    if password == "token"{
       return  Ok(password)
    }
    return Err("token not authorised")
}
fn extract_header_token(reqeust :&ServiceRequest)->Result<String,&'static str>{
    match  reqeust.headers().get("user-token"){
        Some(token) => {
            match token.to_str() {
                Ok(process_token) => Ok(String::from(process_token)),
                Err(_process_token) =>  Err("there is no token"),
            }
        },
        None =>  Err("there is no token"),
    }   
}
pub fn process_token(reqeust :&ServiceRequest)->Result<String,&'static str>{
    match extract_header_token(reqeust) {
        Ok(token) => check_password(token),
        Err(message) => Err(message),
    }
}
