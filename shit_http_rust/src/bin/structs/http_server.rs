use std::{collections::{HashMap, VecDeque}, error::Error, io::{BufRead, BufReader, Write}};

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
            println!("Connection from: {}",stream.peer_addr()?);
            let request = self.read_tcp_stream(&stream)?;
            let response = self.handle_request(request);
            stream.write_all(response.as_bytes())?;
        }
        Ok(())
    }
    pub fn get_config(&self) -> &HashMap<String,String>{
        &self.config
    }
    pub fn get_handler(&self) -> &Box<dyn Handler>{
        &self.handler
    }
    fn read_tcp_stream(&self,mut stream:&std::net::TcpStream) -> Result<VecDeque<String>,Box<dyn Error>>{
        println!("Reading from stream");
        let buf_reader = BufReader::new(&mut stream);
    let request_lines: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("There should be a line to read"))
        .take_while(|s| s.len() > 0)    
        .collect();
        let mut request = vec![];
        for line in request_lines{
            let line = line;
            if line.len() == 0{
                continue;
            }
            request.push(line.trim().to_string());
        };
        Ok(request.iter().filter(|s|s.len() > 0).map(|s|s.to_string()).collect::<VecDeque<String>>() )   
    }

    fn handle_request(&self,request:VecDeque<String>) -> String{
        self.handler.get_response(request)
    }
}