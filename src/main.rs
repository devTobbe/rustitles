use parsers::{parser, srt::SrtParser};
use crate::parsers::parser::SubtitleParser;


mod parsers;


const TEST : &str = r"
1
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

fn main() {
    println!("Hello, world!");
    let parser = SrtParser;

    match parser.parse(TEST) {
        Ok(subtitle) => {
            for caption in subtitle.captions {
                println!("{} --> {}\n{}", caption.start, caption.end, caption.text);
            }
        }
        Err(e) => {
            eprintln!("Error parsing subtitles: {}", e);
        }
    }
}
