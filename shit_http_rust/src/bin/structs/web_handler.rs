use std::{env, error::Error, fs};

use crate::bin::traits::handler::Handler;

pub struct WebHandler{}

impl WebHandler{
    pub fn new() -> WebHandler{
        println!("Creating WebHandler");
        WebHandler{}
    }
    fn get_file_content(path:String) -> Result<String,Box<dyn Error>>{
        let path = match path.as_str() {
            "/" => "/index.html",
            path => path
        };
        let path = format!("{}\\public{}",env::current_dir().expect("There should be a directory.").to_str().unwrap(),path);
        Ok(fs::read_to_string(path)?)
    }
}

impl Handler for WebHandler{
    fn handle_get(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        match WebHandler::get_file_content(path){
            Ok(t) => (httpstatus::StatusCode::Ok,t),
            Err(e) => {
                println!("Error getting file content: {}",e);
                (httpstatus::StatusCode::NotFound,"Error".to_string())
            }
        }
    }

    fn handle_post(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.unhandled(path, req)
    }

    fn handle_put(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.unhandled(path, req)
    }

    fn handle_delete(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.unhandled(path, req)
    }

    fn get_content_type(&self) -> String {
        "text/html".to_string()
    }
}