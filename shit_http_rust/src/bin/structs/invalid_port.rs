use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub struct InvalidPort{
    err:String
}
impl InvalidPort{
    pub fn new() -> InvalidPort{
        InvalidPort{
            err:String::from("An invalid port was used")
        }
    }
    pub fn from(err:String) -> InvalidPort{
        InvalidPort{
            err
        }
    }
}
impl Display for InvalidPort{
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{}",self.err)
    }
}
impl Error for InvalidPort{

}