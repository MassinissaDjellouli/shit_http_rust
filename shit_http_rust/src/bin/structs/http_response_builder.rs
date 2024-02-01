use std::collections::HashMap;

use httpstatus::StatusCode;
pub struct HTTPResponseBuilder{
    headers:HashMap<String,String>,
    body:Option<String>,
    status_code:Option<StatusCode>
}
impl HTTPResponseBuilder{
    pub fn new() -> HTTPResponseBuilder{
        HTTPResponseBuilder{
            headers:HashMap::new(),
            body:None,
            status_code:None
        }
    }
    pub fn add_header(&mut self,key:String,value:String) -> &mut HTTPResponseBuilder{
        self.headers.insert(key,value);
        self
    }
    pub fn set_body(&mut self,body:String) -> &mut HTTPResponseBuilder{
        self.body = Some(body);
        self
    }
    pub fn set_body_from_vec(&mut self,body:Vec<String>) -> &mut HTTPResponseBuilder{
        let body = body.join("\n");
        self.body = Some(body);
        self
    }
    pub fn set_status_code(&mut self,code:StatusCode) -> &mut HTTPResponseBuilder{
        self.status_code = Some(code);
        self
    }
    pub fn build(&self) -> String{
        let code = match self.status_code{
            Some(t) => t,
            None => {
                println!("No status code set, returning 500");
                return "HTTP/1.1 500\r\n\r\nInternal Server Error".to_string();
            }
        };
        let body = match &self.body{
            Some(t) => t,
            None => {
                println!("No body set, returning 500");
                return "HTTP/1.1 500\r\n\r\nInternal Server Error".to_string();
            }
        };
        let mut response = format!("HTTP/1.1 {}\r\n",code);
        for (key,value) in self.headers.iter(){
            response.push_str(&format!("{}: {}\r\n",key,value));
        }
        response.push_str("\r\n");
        response.push_str(body);
        response
    }
}