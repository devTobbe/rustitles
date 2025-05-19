// TEST STRINGS
const TESTSRT: &str = r"1
00:00:01,000 --> 00:00:04,000
Hello, world!

2
00:00:05,000 --> 00:00:07,000
This is a test.

3
00:00:08,500 --> 00:00:12,000
This better be working or I am gonna completely lose it...
This is another line
There can be multiple...
Be ready for that

4
00:00:13,000 --> 00:00:15,000
Let's see if this works.

5
00:00:16,000 --> 00:00:18,000
Did it work..?
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

2
00:00:05,000 --> 00:00:07,000
This is a test.

3
00:00:08,500 --> 00:00:12,000
This better be working or I am gonna completely lose it...
This is another line
There can be multiple...
Be ready for that

4
00:00:13,000 --> 00:00:15,000
Let's see if this works.

5
00:00:16,000 --> 00:00:18,000
Did it work..?
";

const ERRDUMMY : &str = r"
This should just cause a lot of errors >:)
";

#[cfg(test)]
mod tests {
    use std::result;

    use rustitles::{
        format::model::{Caption, Subtitle},
        parse_auto,
    };

    use super::*;

    #[test]
    pub fn test_parse_auto_srt() {
        let sub = Subtitle {
            captions: vec![
                // Caption 1
                Caption::new(
                    "00:00:01,000".to_string(),
                    "00:00:04,000".to_string(),
                    "Hello, world!".to_string(),
                ),
                // Caption 2
                Caption::new(
                    "00:00:05,000".to_string(),
                    "00:00:07,000".to_string(),
                    "This is a test.".to_string(),
                ),
                // Caption 3 (multi-line text)
                Caption::new(
                    "00:00:08,500".to_string(),
                    "00:00:12,000".to_string(),
                    "This better be working or I am gonna completely lose it...\nThis is another line\nThere can be multiple...\nBe ready for that".to_string(),
                ),
                // Caption 4
                Caption::new(
                    "00:00:13,000".to_string(),
                    "00:00:15,000".to_string(),
                    "Let's see if this works.".to_string(),
                ),
                // Caption 5
                Caption::new(
                    "00:00:16,000".to_string(),
                    "00:00:18,000".to_string(),
                    "Did it work..?".to_string(),
                ),
            ],
        };

        let result = parse_auto(&TESTSRT).unwrap();
        assert_eq!(result, sub);
    }

    #[test]
    pub fn test_parse_auto_ass() {
        let sub = Subtitle {
            captions: vec![
                // Caption 1 with formatting tags
                Caption::new(
                    "0:00:01.00".to_string(),
                    "0:00:05.00".to_string(),
                    "{\\i1}Hello, this is a test subtitle in ASS. I am looking forward to writing this parser...".to_string(),
                ),
                // Caption 2 with bold formatting
                Caption::new(
                    "0:00:06.00".to_string(),
                    "0:00:10.00".to_string(),
                    "{\\b1}Another line of text here with bold.".to_string(),
                ),
                // Caption 3 with color formatting
                Caption::new(
                    "0:00:12.00".to_string(),
                    "0:00:15.00".to_string(),
                    "{\\c&HFF0000&}This is the final test line in red.".to_string(),
                ),
            ],
        };

        let result = parse_auto(&TESTASS).unwrap();
        assert_eq!(result, sub);
    }

    #[test]
    pub fn test_parse_auto_vtt() {
        let sub = Subtitle {
            captions: vec![
                // Caption 1
                Caption::new(
                    "00:00:01,000".to_string(),
                    "00:00:04,000".to_string(),
                    "Hello, world!".to_string(),
                ),
                // Caption 2
                Caption::new(
                    "00:00:05,000".to_string(),
                    "00:00:07,000".to_string(),
                    "This is a test.".to_string(),
                ),
                // Caption 3 (multi-line text)
                Caption::new(
                    "00:00:08,500".to_string(),
                    "00:00:12,000".to_string(),
                    "This better be working or I am gonna completely lose it...\nThis is another line\nThere can be multiple...\nBe ready for that".to_string(),
                ),
                // Caption 4
                Caption::new(
                    "00:00:13,000".to_string(),
                    "00:00:15,000".to_string(),
                    "Let's see if this works.".to_string(),
                ),
                // Caption 5
                Caption::new(
                    "00:00:16,000".to_string(),
                    "00:00:18,000".to_string(),
                    "Did it work..?".to_string(),
                ),
            ],
        };

        let result = parse_auto(&TESTVTT).unwrap();
        assert_eq!(result, sub);
    }

    #[test]
    #[should_panic]
    fn wrong_format_panic() {
       let _ = parse_auto(&ERRDUMMY);
    }

}
