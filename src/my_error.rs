use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    message: String,
}
impl MyError {
    pub fn new(whats_wrong: String) -> Self {
        MyError {
            message: whats_wrong,
        }
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for MyError {}

pub fn build_error(reason: String) -> Result<String, Box<dyn Error>> {
    Err(Box::new(MyError::new(format!(
        "{}: {:?}",
        reason,
        env::args().collect::<Vec<String>>()
    ))))
}
