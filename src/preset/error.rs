use std::{fmt, io};

#[derive(Debug)]
pub enum ParseError {
    Toml(toml::de::Error),
    Json(serde_json::error::Error),
    Yaml(serde_yaml::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = self;
        write!(f, "{e}")
    }
}

#[derive(Debug)]
pub enum LoadError {
    Read(io::Error),
    Parse(ParseError),
    InvalidExtension,
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        LoadError::Read(value)
    }
}

impl From<toml::de::Error> for LoadError {
    fn from(value: toml::de::Error) -> Self {
        LoadError::Parse(ParseError::Toml(value))
    }
}

impl From<serde_json::error::Error> for LoadError {
    fn from(value: serde_json::error::Error) -> Self {
        LoadError::Parse(ParseError::Json(value))
    }
}

impl From<serde_yaml::Error> for LoadError {
    fn from(value: serde_yaml::Error) -> Self {
        LoadError::Parse(ParseError::Yaml(value))
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Read(e) => write!(f, "Couldn't read config file: {e}"),
            LoadError::Parse(e) => write!(f, "Couldn't parse config file: {e}"),
            LoadError::InvalidExtension => write!(f, "Invalid config file extension"),
        }
    }
}
