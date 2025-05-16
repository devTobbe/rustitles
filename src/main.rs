mod format;

use crate::format::SubtitleParser;
use format::{ass::AssParser, srt::SrtParser, vtt::VttParser};

const TESTSRT: &str = r"1
00:00:01,000 --> 00:00:03,000
Hello, welcome to the video.

2
00:00:03,500 --> 00:00:06,000
In this part, we'll learn something new.

3
00:00:07,000 --> 00:00:09,000
Let's get started!
Yiiihaaa
";

const TESTASS: &str = r"[Script Info]
Title: Sample Subtitle
ScriptType: v4.00+
Collisions: Normal
PlayResX: 1920
PlayResY: 1080
Timer: 100.0000

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,36,&H00FFFFFF,&H0000FFFF,&H00000000,&H64000000,-1,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello, this is a test subtitle.
Dialogue: 0,0:00:05.00,0:00:08.00,Default,,0,0,0,,Second line appears now.
Dialogue: 0,0:00:09.00,0:00:12.00,Default,,0,0,0,,Final test line.
";

const TESTVTT: &str = r"WEBVTT

1
00:00:01.000 --> 00:00:04.000
Hello, this is a test subtitle.

2
00:00:05.000 --> 00:00:08.000
Second line appears now.

3
00:00:09.000 --> 00:00:12.000
Final test line.

";

fn main() {
    println!("Hello, world!");
    let srtparser = SrtParser;
    let vttparser = VttParser;
    let assparser = AssParser;

    println!("SRT:");
    match srtparser.parse(TESTSRT) {
        Ok(subtitle) => {
            for caption in subtitle.captions {
                println!("{} --> {}\n{}", caption.start, caption.end, caption.text);
                println!()
            }
        }
        Err(e) => {
            eprintln!("Error parsing subtitles: {}", e);
        }
    }

    println!("ASS:");
    match assparser.parse(TESTASS) {
        Ok(subtitle) => {
            for caption in subtitle.captions {
                println!("{} --> {}\n{}", caption.start, caption.end, caption.text);
                println!()
            }
        }
        Err(e) => {
            eprintln!("Error parsing subtitles: {}", e);
        }
    }

    println!("VTT:");
    match vttparser.parse(TESTVTT) {
        Ok(subtitle) => {
            for caption in subtitle.captions {
                println!("{} --> {}\n{}", caption.start, caption.end, caption.text);
                println!()
            }
        }
        Err(e) => {
            eprintln!("Error parsing subtitles: {}", e);
        }
    }
}
