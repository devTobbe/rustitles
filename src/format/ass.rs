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
        let re = match Regex::new(ASS_PATTERN) {
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
