use crate::bin::traits::handler::Handler;

struct WebHandler{}

impl Handler for WebHandler{
    fn handle(&self,req:Vec<String>) -> String {
        return ;
    }
}