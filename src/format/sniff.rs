use regex::Regex;

use crate::format::model::FormatError;
use crate::format::model::SubFormat;

const SRT_SNIFF: &str = r"^\d+\r?\n\d+:\d+:\d+,\d+ --> \d+:\d+:\d+,\d+\r?\n";
const VTT_SNIFF: &str = r"^WEBVTT";
const ASS_SNIFF: &str = r"^\[Script Info\]";

// Sniffs out the format of the inserted string pointer using detection regex
pub fn sniff_format(input: &str) -> Result<SubFormat, FormatError> {
    let re_srt = Regex::new(SRT_SNIFF).unwrap();
    let re_ass = Regex::new(ASS_SNIFF).unwrap();
    let re_vtt = Regex::new(VTT_SNIFF).unwrap();

    //VTT has to be first I think. This is horrible. Goblin ah programming
    if re_vtt.is_match(input) {
        return Ok(SubFormat::VTT);
    };
    if re_ass.is_match(input) {
        return Ok(SubFormat::ASS);
    };
    if re_srt.is_match(input) {
        return Ok(SubFormat::SRT);
    };

    return Err(FormatError::UnsupportedFormat);
}
