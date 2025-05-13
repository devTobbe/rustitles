use regex::Regex;

use crate::model::SubFormat;

const SRT_SNIFF : &str = r"WEBVTT";
const ASS_SNIFF : &str = r"^\[Script Info\]";
const VTT_SNIFF : &str = r"^\d+\r?\n\d+:\d+:\d+,\d+ --> \d+:\d+:\d+,\d+\r?\n";

pub fn sniff_format(input : &str) -> Option<SubFormat> {
    let re_srt = Regex::new(SRT_SNIFF).unwrap();
    let re_ass = Regex::new(ASS_SNIFF).unwrap();
    let re_vtt = Regex::new(VTT_SNIFF).unwrap();

    //VTT has to be first I think. This is horrible.
    if re_vtt.is_match(input) { return Some(SubFormat::VTT) };
    if re_ass.is_match(input) { return Some(SubFormat::ASS) };
    if re_srt.is_match(input) { return Some(SubFormat::SRT) };

    return None;

}
