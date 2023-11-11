use std::io;
#[derive(Debug)]
pub enum Errors {
    // Wrapped error from io::error
    IOError(io::Error),
}
impl From<io::Error> for Errors {
    fn from(e: io::Error) -> Self {
        Errors::IOError(e)
    }
}
