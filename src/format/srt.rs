use regex::Regex;
use std::io::Error;

use crate::format::SubtitleParser;
use crate::format::model::Caption;
use crate::format::model::Subtitle;

const SRT_PATTERN: &str = r"(?m)^\d+?\n(\d{2}:\d{2}:\d{2}[.,]\d{3})\s+-->\s+(\d{2}:\d{2}:\d{2}[.,]\d{3})(?:[^\n])*\n((?:[^\n]+\n?)+)";

pub struct SrtParser;

// TODO: Error handling
impl SubtitleParser for SrtParser {
    // Parser logic for SRT files
    fn parse(&self, input: &str) -> Result<Subtitle, Error> {
        let mut subs = Subtitle::new();
        let re = Regex::new(SRT_PATTERN).unwrap();
        for caps in re.captures_iter(input) {
            let caption = Caption::new(
                caps[1].to_string(),
                caps[2].to_string(),
                caps[3].trim().to_string(),
            );
            subs.captions.push(caption);
        }
        Ok(subs)
    }
}
