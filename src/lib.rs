// Rustitles library

use std::io::Error;

use format::{
    SubtitleParser,
    ass::AssParser,
    model::{SubFormat, Subtitle},
    sniff::sniff_format,
    srt::SrtParser,
    vtt::VttParser,
};

pub mod format;

/// Automatically detects the format of the file and returns a Result with 
/// a Subtitle structure or an error.
pub fn parse_auto(input: &str) -> Result<Subtitle, Error> {
    match sniff_format(input) {
        Ok(SubFormat::SRT) => {
            let srtprs = SrtParser;
            srtprs.parse(input)
        },
        Ok(SubFormat::ASS) => {
            let assprs = AssParser;
            assprs.parse(input)
        },
        Ok(SubFormat::VTT) => {
            let vttprs = VttParser;
            vttprs.parse(input)
        },
        Err(error) => {
            panic!("{error}")
        }
    }
}

