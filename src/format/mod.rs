pub mod ass;
pub mod model;
pub mod sniff;
pub mod srt;
pub mod vtt;

pub use self::model::{Caption, SubFormat, Subtitle};
pub use self::sniff::sniff_format;

use std::io::Error;

// TODO: Add these to clean subtitles later on... Maybe?
//const TAG_PATTERN: &str = r"<[^>]+>";
//const BRACKET_PATTERN: &str = r"{[^}]+}";

pub trait SubtitleParser {
    fn parse(&self, input: &str) -> Result<Subtitle, Error>;
}
