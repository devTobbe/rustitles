
const TAG_PATTERN : String = r"<[^>]+>";
const BRACKET_PATTERN : String = r"{[^}]+}";

pub trait SubtitleParser {
    fn parse(&self, input: String) -> Result<Vec<Subtitle>, Error>
}
