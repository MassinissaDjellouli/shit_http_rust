use crate::bin::traits::handler::Handler;

use super::routes::RoutesBuilder;

pub struct RestHandler{
    routes:RoutesBuilder
}

impl RestHandler{
    pub fn new(routes:RoutesBuilder) -> RestHandler{
        println!("Creating RestHandler");
        RestHandler{
            routes
        }   
    }
    pub 
    fn handle_method(&self,method:String,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String){
        match self.routes.get(format!("{}@{}",method,path).as_str()){
            Some(t) => t(req),
            None => (httpstatus::StatusCode::NotFound,"Error".to_string())
        }
    }
}

impl Handler for RestHandler{
    fn handle_get(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.handle_method("GET".to_string(), path, req)
    }

    fn handle_post(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.handle_method("POST".to_string(), path, req)
    }

    fn handle_put(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.handle_method("PUT".to_string(), path, req)
    }

    fn handle_delete(&self,path:String,req:std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String) {
        self.handle_method("DELETE".to_string(), path, req)
    }

    fn get_content_type(&self) -> String {
        "text/html".to_string()
    }
}