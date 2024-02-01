use std::{collections::{HashMap, VecDeque}, env, error::Error, fmt::format, fs, io::{BufRead, Write}, process::exit};
use crate::bin::{structs::{empty_or_missing_config::EmptyOrMissingConfig,http_server::HTTPServer}, traits::handler::{self, Handler}};
pub fn create() -> HTTPServer{
    match unhandled_create(){
        Ok(t) => t,
        Err(e) => {
            default_handle_error(e);
            exit(0);
        }
    }
}
pub fn unhandled_create() -> Result<HTTPServer,Box<dyn Error>>{
    let config = parse_config(get_config_path())?;
    let port = match config.get("port"){
        Some(t) => match t.parse::<i32>(){
            Ok(t) => t,
            Err(e) => return Err(Box::new(e))
        },
        None => 8080
    };
    let handler = match config.get("handler"){
        Some(t) => match t.as_str(){
            "web" => 
            _ => {
                println!("Handler '{}' not found, using default",t);
                
            }
        },
        None => Box::new(handler::EchoHandler::new()) as Box<dyn Handler>
    };
    HTTPServer::new(port,config)
}
pub fn try_create() -> Option<HTTPServer>{
    match unhandled_create(){
        Ok(t) => Some(t),
        Err(e) => None
    }
}
fn get_config_path() -> String{
    match env::current_dir() {
        Ok(t) => {
            let mut path = t.into_os_string().into_string().expect("Could not convert path to string");
            path.push_str("/config/start.conf");
            return path;
        },
        Err(e) => {
            println!("Could not get current directory: {}",e);
        }        
    }
    "/shithttpserver/config/start.conf".to_string()
}
fn parse_config(conf_path:String) -> Result<HashMap<String,String>,Box<dyn Error>>{
    let config = fs::read_to_string(conf_path)?;
    let config = config.split("\n").collect::<Vec<&str>>();
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
