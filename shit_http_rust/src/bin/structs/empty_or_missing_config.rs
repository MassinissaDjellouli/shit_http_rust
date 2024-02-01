use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub struct EmptyOrMissingConfig{
    err:String
}
impl EmptyOrMissingConfig{
    pub fn new() -> EmptyOrMissingConfig{
        EmptyOrMissingConfig{
            err:String::from("No valid configuration file found for the server")
        }
    }
    pub fn from(err:String) -> EmptyOrMissingConfig{
        EmptyOrMissingConfig{
            err
        }
    }
}
impl Display for EmptyOrMissingConfig{
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{}",self.err)
    }
}
impl Error for EmptyOrMissingConfig{

}