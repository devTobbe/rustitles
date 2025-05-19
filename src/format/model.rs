use std::{error::Error, fmt::Display};

// Supported Subtitle formats
#[derive(Debug, PartialEq)]
pub enum SubFormat {
    SRT,
    ASS,
    VTT,
}

// Structure for a single caption
#[derive(Debug, PartialEq)]
pub struct Caption {
    pub start: String,
    pub end: String,
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub enum FormatError {
    UnsupportedFormat,
    UnrecognizeableFormatting,
}

impl Error for FormatError {}

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnsupportedFormat => {
                "Unsupported file format, library currently supports: srt, ass, ssa, vtt."
            }
            Self::UnrecognizeableFormatting => {
                "Formatting of file is urecognizeabe, make sure that you provided a properly formatted file."
            }
        };
        write!(f, "Error: {message}")
    }
}

// Structure for a subttile, which contains a vector of Captions
#[derive(Debug, PartialEq)]
pub struct Subtitle {
    pub captions: Vec<Caption>,
}

impl Caption {
    pub fn new(start: String, end: String, text: String) -> Self {
        Self { start, end, text }
    }
}

impl Subtitle {
    pub fn new() -> Self {
        Self { captions: vec![] }
    }
}
