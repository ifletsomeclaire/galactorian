use std::error::Error;

#[derive(Debug)]
pub enum GalError {
    ParseError(String),
}
impl std::fmt::Display for GalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}
impl Error for GalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl From<std::io::Error> for GalError {
    fn from(e: std::io::Error) -> Self {
        GalError::ParseError(e.to_string())
    }
}
impl From<toml::de::Error> for GalError {
    fn from(e: toml::de::Error) -> Self {
        GalError::ParseError(e.to_string())
    }
}
