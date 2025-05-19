use regex::Regex;
use std::io::Error;

use crate::format::model::FormatError;
use crate::format::SubtitleParser;
use crate::format::model::Caption;
use crate::format::model::Subtitle;

const VTT_PATTERN: &str = r"(?m)^(?:WEBVTT\s*\n{1,2})?^\d+\n(\d{2}:\d{2}:\d{2}[.,]\d{3})\s*-->\s*(\d{2}:\d{2}:\d{2}[.,]\d{3})(?:[^\n]*)\n((?:[^\n]+\n?)*)";

pub struct VttParser;

// TODO: Error handling
impl SubtitleParser for VttParser {
    // Parser logic for VTT files
    fn parse(&self, input: &str) -> Result<Subtitle, Error> {
        let mut subs = Subtitle::new();
        let re = match Regex::new(VTT_PATTERN) {
            Ok(result) => result,
            Err(_) => {
                let error = FormatError::UnrecognizeableFormatting;
                panic!("{error}");
            }
        };
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
