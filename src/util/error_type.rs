use std::io;
#[derive(Debug)]
pub enum Errors {
    // Wrapped error from io::error
    IOError(io::Error),
    ErrorReadingFile(io::Error),
    SerdeJsonError(serde_json::Error),
}
impl From<io::Error> for Errors {
    fn from(e: io::Error) -> Self {
        Errors::IOError(e)
    }
}
impl From<serde_json::Error> for Errors {
    fn from(e: serde_json::Error) -> Self {
        Errors::SerdeJsonError(e)
    }
}
