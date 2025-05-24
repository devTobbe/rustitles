use regex::Regex;
use std::io::Error;

use crate::format::SubtitleParser;
use crate::format::model::Caption;
use crate::format::model::FormatError;
use crate::format::model::Subtitle;

const ASS_PATTERN: &str = r"(?m)^Dialogue:\s*(?:|\w+\W)(?:Marked=\d+,\s*)?\d+,(?P<StartTime>\d+:\d+:\d+\.\d+),(?P<EndTime>\d+:\d+:\d+\.\d+),[^,]*,[^,]*,\d+,\d+,\d+,[^,]*,(?P<Content>.+)$";

pub struct AssParser;

impl SubtitleParser for AssParser {
    // Parser logic for ASS files
    fn parse(&self, input: &str) -> Result<Subtitle, Error> {
        let mut subs = Subtitle::new();
        let re = match Regex::new(ASS_PATTERN) {
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
