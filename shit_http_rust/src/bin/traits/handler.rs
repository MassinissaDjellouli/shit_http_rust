use std::fmt::format;

use crate::bin::structs::http_response_builder::HTTPResponseBuilder;

pub trait Handler{
    fn handle(&self,req:Vec<String>) -> String{
        let methodFlag = getMethodFlag(method);
    }
    fn handleGet(&self,req:Vec<String>) -> String;
    fn handlePost(&self,req:Vec<String>) -> String;
    fn handlePut(&self,req:Vec<String>) -> String;
    fn handleDelete(&self,req:Vec<String>) -> String;
    fn unhandled(&self,req:Vec<String>) -> String {
        "Method not handled".to_string()
    }
    fn getContentType(&self) -> String;
    fn getResponse(&self,req:Vec<String>) -> String{
        let body = self.handle(req);
        HTTPResponseBuilder::new()
        .add_header("Server".to_string(), "ShitHTTP".to_string())
        .add_header("Content-Length".to_string(),format(format_args!("{}",body.len())))
        .add_header("Content-Type".to_string(), self.getContentType())
        .set_body(body)
        .build()
    }
}