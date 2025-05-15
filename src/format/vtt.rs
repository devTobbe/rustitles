use regex::Regex;
use std::io::Error;

use crate::format::SubtitleParser;
use crate::format::model::Caption;
use crate::format::model::Subtitle;

const VTT_PATTERN: &str = r"(?m)^(?:WEBVTT\n)?(?:\d+\n)?(\d{2}:\d{2}:\d{2}\.\d{3})\s*-->\s*(\d{2}:\d{2}:\d{2}\.\d{3})\n((?:[^\n]+\n?)*)\n";

pub struct VttParser;

// TODO: Error handling
impl SubtitleParser for VttParser {
    // Parser logic for VTT files
    fn parse(&self, input: &str) -> Result<Subtitle, Error> {
        let mut subs = Subtitle::new();
        let re = Regex::new(VTT_PATTERN).unwrap();
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
