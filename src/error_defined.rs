
use std::{error,fmt};
#[derive(Debug,Default)]
pub struct ErrorA {}
impl fmt::Display for ErrorA{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"ErrorA!")
    }
}
impl error::Error for ErrorA{
    fn description(&self) -> &str{
        "Description for ErrorA"
    }
    fn cause(&self) -> Option<&error::Error>{
        None
    }
}
#[derive(Debug,Default)]
pub struct ErrorB{}
impl fmt::Display for ErrorB{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f,"ErrorB!")
    }
}
impl error::Error for ErrorB{
    fn description(&self)->&str{
        "Description for ErrorB!"
    }
    fn cause(&self)->Option<&error::Error>{
        None
    }
}