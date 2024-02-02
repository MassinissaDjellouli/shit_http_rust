use std::{collections::HashMap, ops::Deref};

pub struct RoutesBuilder{
    routes:HashMap<String,Box<dyn Fn(std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String)>>
}

impl RoutesBuilder{
    pub fn new() -> RoutesBuilder{
        RoutesBuilder{
            routes:HashMap::new()
        }   
    }
    pub fn add_route(&mut self,method:&str,path:&str,handler:Box<dyn Fn(std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String)>) -> &mut RoutesBuilder{
        self.routes.insert(format!("{}@{}",method,path),handler);
        self
    }

    pub fn get(&self,key:&str) -> Option<&Box<dyn Fn(std::collections::VecDeque<String>) -> (httpstatus::StatusCode,String)>>{
        self.routes.get(key)
    }
}