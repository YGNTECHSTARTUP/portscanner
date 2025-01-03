use thiserror::Error;
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Usage Scanport: <Domain.name>")]
    CliError,
    #[error("Reqwest {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}
