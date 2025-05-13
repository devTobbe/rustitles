pub mod ass;
pub mod srt;
pub mod vtt;

use crate::Subtitle;
use std::io::Error;

const TAG_PATTERN: &str = r"<[^>]+>";
const BRACKET_PATTERN: &str = r"{[^}]+}";

pub trait SubtitleParser {
    fn parse(&self, input: &str) -> Result<Subtitle, Error>;
}
