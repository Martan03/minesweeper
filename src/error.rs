use std::fmt::Display;

pub enum Error {
    IOErr(std::io::Error),
    Msg(String),
    Exit,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOErr(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Msg(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOErr(e) => write!(f, "{e}"),
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::Exit => write!(f, "exit"),
        }
    }
}
