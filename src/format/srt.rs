use regex::Regex;
use std::io::Error;

use crate::format::SubtitleParser;
use crate::format::model::Caption;
use crate::format::model::FormatError;
use crate::format::model::Subtitle;

const SRT_PATTERN: &str = r"(?m)^\d+?\n(\d{2}:\d{2}:\d{2}[.,]\d{3})\s+-->\s+(\d{2}:\d{2}:\d{2}[.,]\d{3})(?:[^\n])*\n((?:[^\n]+\n?)+)";

pub struct SrtParser;

impl SubtitleParser for SrtParser {
    // Parser logic for SRT files
    fn parse(&self, input: &str) -> Result<Subtitle, Error> {
        //let mut subs = Subtitle::new();
        let re = match Regex::new(SRT_PATTERN) {
            Ok(result) => result,
            Err(_) => {
                let error = FormatError::UnrecognizeableFormatting;
                panic!("{error}");
            }
        };

        let captions = re
            .captures_iter(input)
            .map(|caps| {
                Caption::new(
                    caps[1].to_string(),
                    caps[2].to_string(),
                    caps[3].trim().to_string(),
                )
            })
            .collect();

        Ok(Subtitle { captions })
    }
}
