use std::{io, string};

#[derive(Debug)]
pub enum ExtractError {
    NoFileExtensionError,
    FailedToExtract,
    SevenZNotFound,
    FailedToInstall7zip,
    UnsupportedArchive(String),
    IOerror(io::Error),

    // HIGHLY UNLIKELY
    NoFileNameError,
    OuputParseError(string::FromUtf8Error),
    OsStrConversionError,
    StdErr(String),
}

impl From<io::Error> for ExtractError {
    fn from(err: io::Error) -> Self {
        Self::IOerror(err)
    }
}

impl From<string::FromUtf8Error> for ExtractError {
    fn from(err: string::FromUtf8Error) -> Self {
        Self::OuputParseError(err)
    }
}
