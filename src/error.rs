#[derive(Debug)]
pub enum Error {
    BadResponse,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}
