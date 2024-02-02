use std::{collections::{HashMap, VecDeque}, env, error::Error, fs, process::exit};
use crate::bin::{structs::{empty_or_missing_config::EmptyOrMissingConfig, http_server::HTTPServer, rest_handler::RestHandler, routes::RoutesBuilder, web_handler::WebHandler}, traits::handler::{self, Handler}};
pub fn create(config:Option<String>,routes:Option<RoutesBuilder>) -> HTTPServer{
    println!("{}",get_config_path());
    match unhandled_create(config,routes){
        Ok(t) => t,
        Err(e) => {
            default_handle_error(e);
            exit(0);
        }
    }
}
pub fn unhandled_create(config:Option<String>,routes:Option<RoutesBuilder>) -> Result<HTTPServer,Box<dyn Error>>{
    let config = match config{
        Some(t) => parse_config(t)?,
        None => parse_config(fs::read_to_string(get_config_path())?)?
    };
    let port = match config.get("port"){
        Some(t) => match t.parse::<i32>(){
            Ok(t) => t,
            Err(e) => return Err(Box::new(e))
        },
        None => 8080
    };
    println!("Using port {}",port);
    let handler:Box<dyn Handler> = match config.get("handler"){
        Some(t) => match t.as_str(){
            "web" => Box::new(WebHandler::new()), 
            "rest" => Box::new(RestHandler::new(routes.expect("Routes not found."))),
            _ => {
                println!("Handler '{}' not found, using default",t);
                Box::new(WebHandler::new())
            }
        },
        None => Box::new(WebHandler::new())
    };
    HTTPServer::new(port,config,handler)
}
pub fn try_create(config:Option<String>,routes:Option<RoutesBuilder>) -> Option<HTTPServer>{
    match unhandled_create(config,routes){
        Ok(t) => Some(t),
        Err(e) => None
    }
}
fn get_config_path() -> String{
    match env::current_dir() {
        Ok(t) => {
            let mut path = t.into_os_string().into_string().expect("Could not convert path to string");
            path.push_str("\\config\\start.conf");
            return path;
        },
        Err(e) => {
            println!("Could not get current directory: {}",e);
        }        
    }
    "/shithttpserver/config/start.conf".to_string()
}
pub fn parse_config(config:String) -> Result<HashMap<String,String>,Box<dyn Error>>{
    let config = config.split("\n").map(|s|s.trim()).filter(|s|!s.is_empty()).collect::<Vec<&str>>();
    println!("{:?}",config);
    let mut map: HashMap<String,String>= HashMap::new();
    for line in config {
        let split_iter = line.split("=");
        let mut key_val = split_iter.collect::<VecDeque<&str>>();
        let key = match key_val.pop_front(){
            Some(t) => match t.trim().len(){
                0 => return Err(Box::new(EmptyOrMissingConfig::new())),
                _ => t
            },
            None => return Err(Box::new(EmptyOrMissingConfig::new()))
        };
        map.insert(
            key.to_string(),
            key_val.iter().map(|s|s.to_string()).collect::<Vec<String>>().join("=").trim().to_string()
        );
    }
    println!("{:?}",map);
    Ok(map)
}
fn default_handle_error(e:Box<dyn std::error::Error>){
    println!("Server could not be created: \n{}",e);
}
