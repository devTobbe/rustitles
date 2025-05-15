// Supported Subtitle formats
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
