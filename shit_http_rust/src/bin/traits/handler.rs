use std::{collections::VecDeque, fmt::format};

use crate::bin::structs::http_response_builder::HTTPResponseBuilder;

pub trait Handler{
    fn handle(&self,mut req:VecDeque<String>) -> (httpstatus::StatusCode,String){
        let first_line = req.pop_front();
        let first_line = match first_line {
            Some(s) => s,
            None => return (httpstatus::StatusCode::BadRequest,"Error".to_string())
        };
        let mut first_line = first_line.split(" ").map(|s| s.to_string()).collect::<VecDeque<String>>();
        let method = match first_line.pop_front(){
            Some(s) => s,
            None => return (httpstatus::StatusCode::BadRequest,"Error".to_string())
        };
        match method.as_str(){
            "GET" => self.handle_get(first_line.pop_front().unwrap(),req),
            "POST" => self.handle_post(first_line.pop_front().unwrap(),req),
            "PUT" => self.handle_put(first_line.pop_front().unwrap(),req),
            "DELETE" => self.handle_delete(first_line.pop_front().unwrap(),req),
            _ => self.unhandled(first_line.pop_front().unwrap(),req)
        }
    }
    fn handle_get(&self,path:String,req:VecDeque<String>) -> (httpstatus::StatusCode,String);
    fn handle_post(&self,path:String,req:VecDeque<String>) -> (httpstatus::StatusCode,String);
    fn handle_put(&self,path:String,req:VecDeque<String>) -> (httpstatus::StatusCode,String);
    fn handle_delete(&self,path:String,req:VecDeque<String>) -> (httpstatus::StatusCode,String);
    fn unhandled(&self,path:String,req:VecDeque<String>) -> (httpstatus::StatusCode,String) {
        (httpstatus::StatusCode::MethodNotAllowed,"Method not allowed".to_string())
    }
    fn get_content_type(&self) -> String;
    fn get_response(&self,req:VecDeque<String>) -> String{
        let (status,body) = self.handle(req);
        HTTPResponseBuilder::new()
        .add_header("Server".to_string(), "ShitHTTP".to_string())
        .add_header("Content-Length".to_string(),format(format_args!("{}",body.len())))
        .add_header("Content-Type".to_string(), self.get_content_type())
        .set_body(body)
        .set_status_code(status)
        .build()
    }
}