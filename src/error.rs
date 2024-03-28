pub enum Error {
    IOErr(std::io::Error),
    ExitErr,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOErr(value)
    }
}
