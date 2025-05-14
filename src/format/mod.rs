pub mod ass;
pub mod srt;
pub mod vtt;
pub mod sniff;
pub mod model;

use model::Subtitle;
use std::io::Error;

// TODO: Add these to clean subtitles later on... Maybe?
//const TAG_PATTERN: &str = r"<[^>]+>";
//const BRACKET_PATTERN: &str = r"{[^}]+}";

pub trait SubtitleParser {
    fn parse(&self, input: &str) -> Result<Subtitle, Error>;
}
