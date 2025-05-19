const TESTSRT: &str = r"1
00:00:01,000 --> 00:00:04,000
Hello, world!
";

const TESTASS: &str = r"[Script Info]
Title: Example ASS Subtitle
Original Script: Your Name
ScriptType: v4.00+
Collisions: Normal
PlayDepth: 0
Timer: 100.0000

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,24,&H00FFFFFF,&H000000FF,&H00000000,&H64000000,-1,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1
Style: Highlight,Arial,24,&H00FFFF00,&H000000FF,&H00000000,&H64000000,-1,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:05.00,Default,,0,0,0,,{\i1}Hello, this is a test subtitle in ASS. I am looking forward to writing this parser...
Dialogue: 0,0:00:06.00,0:00:10.00,Default,,0,0,0,,{\b1}Another line of text here with bold.
Dialogue: 0,0:00:12.00,0:00:15.00,Highlight,,0,0,0,,{\c&HFF0000&}This is the final test line in red.
";

const TESTVTT: &str = r"WEBVTT

1
00:00:01,000 --> 00:00:04,000
Hello, world!
";

#[cfg(test)]
mod tests {
    use rustitles::format::{model::{FormatError, SubFormat}, sniff::sniff_format};

    use super::*;

    #[test]
    pub fn test_sniff_srt() {
        let result = sniff_format(&TESTSRT);

        assert_eq!(result.unwrap(), SubFormat::SRT);
   }

    #[test]
    pub fn test_sniff_ass() {
        let result = sniff_format(&TESTASS);

        assert_eq!(result.unwrap(), SubFormat::ASS);
    }

    #[test]
    pub fn test_sniff_vtt() {
        let result = sniff_format(&TESTVTT);

        assert_eq!(result.unwrap(), SubFormat::VTT);
    }

    #[test]
    pub fn test_sniff_err() {
        let input = r"THIS SHOULD FAIL";
        assert_eq!(sniff_format(input), Err(FormatError::UnsupportedFormat));

    }
}
