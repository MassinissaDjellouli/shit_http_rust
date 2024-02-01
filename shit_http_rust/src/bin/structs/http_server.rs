use std::{collections::HashMap, error::Error, io::{BufRead, Write}};

use crate::bin::traits::handler::{self, Handler};

use super::invalid_port::InvalidPort;

pub struct HTTPServer{
    port:i32,
    config:HashMap<String,String>,
    handler:Box<dyn Handler>
}
impl HTTPServer{
    pub fn new(port:i32,config:HashMap<String,String>,handler:Box<dyn Handler>) -> Result<HTTPServer,Box<dyn Error>>{
        if port < 1024 || port > 65535{
            return Err(Box::new(InvalidPort::new()));
        }
        Ok(HTTPServer{
            port,
            config,
            handler
        })
    }
    pub fn start_listening(&self){
        match self.unhandled_start_listening(){
            Ok(_) => (),
            Err(e) => {
                println!("Server could not start listening: {}",e);
            }
        }
    }
    
    pub fn unhandled_start_listening(&self) -> Result<(),Box<dyn Error>>{
        println!("Server is listening on port {}",self.port);
        let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}",self.port))?;
        for stream in listener.incoming(){
            let mut stream = stream?;
            let request = self.read_tcp_stream(&stream)?;
            let response = self.handle_request(request);
            stream.write_all(response.as_bytes())?;
        }
        
        Ok(())
    }
    pub fn override_config(&mut self,config:HashMap<String,String>){
        self.config = config;
    }
    
    fn read_tcp_stream(&self,stream:&std::net::TcpStream) -> Result<Vec<String>,Box<dyn Error>>{
        let buf_reader = std::io::BufReader::new(stream);
        let request_lines = buf_reader.lines().collect::<Vec<_>>();
        let mut request = vec![];
        for line in request_lines{
            let line = line?;
            if line.len() == 0{
                continue;
            }
            request.push(line.trim().to_string());
        };
        Ok(request.iter().filter(|s|s.len() > 0).map(|s|s.to_string()).collect::<Vec<String>>() )   
    }

    fn handle_request(&self,request:Vec<String>) -> String{
        return "TEST".to_string();
    }
}