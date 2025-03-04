use std::{fmt::Display, io, string};

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

impl Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ExtractError::NoFileExtensionError => {
                "No file extension found for the provided archive"
            }
            ExtractError::FailedToExtract => "Failed to extract the archive",
            ExtractError::SevenZNotFound => "7zip not found in PATH",
            ExtractError::FailedToInstall7zip => "Failed to install 7zip",
            ExtractError::UnsupportedArchive(filetype) => {
                &(String::from("invalid archive type: ") + filetype)
            }
            ExtractError::IOerror(error) => &error.to_string(),
            ExtractError::NoFileNameError => todo!("Archive does not have a file name (rare)"),
            ExtractError::OuputParseError(error) => &error.to_string(),
            ExtractError::OsStrConversionError => todo!("Failed to convert OS string"),
            ExtractError::StdErr(error) => error,
        };

        write!(f, "{text}")
    }
}
