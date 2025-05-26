mod util;

#[cfg(test)]
mod tests {
    

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

        let file = util::read_file("tests/data/basic.srt");
        let result = parse_auto(&file).unwrap();
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

        let file = util::read_file("tests/data/basic.ass");
        let result = parse_auto(&file).unwrap();
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

        let file = util::read_file("tests/data/basic.vtt");
        let result = parse_auto(&file).unwrap();
        assert_eq!(result, sub);
    }

    #[test]
    #[should_panic]
    fn wrong_format_panic() {
        let file = util::read_file("tests/data/basic.non");
        let _ = parse_auto(&file);
    }
}
