use std::io::Error;
use crate::parsers::types::Subtitle;

const TAG_PATTERN : &str = r"<[^>]+>";
const BRACKET_PATTERN : &str = r"{[^}]+}";

pub trait SubtitleParser {
    fn parse(&self, input: &str) -> Result<Subtitle, Error>;
}
