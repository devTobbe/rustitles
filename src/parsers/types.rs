
enum SubFormat {
    SRT,
    ASS,
    SSA,
    VTT
}

#[derive(Debug)]
pub struct Caption {
    pub start: String,
    pub end: String,
    pub text: String,
}

#[derive(Debug)]
pub struct Subtitle {
    pub captions : Vec<Caption> 
}

impl Caption {
    pub fn new(start : String, end : String, text : String) -> Self {
        Self {
            start,
            end,
            text,
        }
    }
}

impl Subtitle {
    pub fn new() -> Self {
        Self {
            captions: vec![],
        }
    }
}
