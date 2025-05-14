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

pub fn parse_auto(input: &str) -> Result<Subtitle, Error> {
    match sniff_format(input) {
        Some(SubFormat::SRT) => {
            let srtprs = SrtParser;
            srtprs.parse(input)
        }
        Some(SubFormat::ASS) => {
            let assprs = AssParser;
            assprs.parse(input)
        }
        Some(SubFormat::VTT) => {
            let vttprs = VttParser;
            vttprs.parse(input)
        }
        // TODO: Error handling, remove the srtparser later
        None => {
            let srtprs = SrtParser;
            srtprs.parse(input)
        }
    }
}

