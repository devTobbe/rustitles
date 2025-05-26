mod util;

#[cfg(test)]
mod tests {
    use rustitles::format::{
        model::{FormatError, SubFormat},
        sniff::sniff_format,
    };

    use super::*;

    #[test]
    pub fn test_sniff_srt() {
        let file = util::read_file("tests/data/basic.srt");
        let result = sniff_format(&file);

        assert_eq!(result.unwrap(), SubFormat::SRT);
    }

    #[test]
    pub fn test_sniff_ass() {
        let file = util::read_file("tests/data/basic.ass");
        let result = sniff_format(&file);

        assert_eq!(result.unwrap(), SubFormat::ASS);
    }

    #[test]
    pub fn test_sniff_vtt() {
        let file = util::read_file("tests/data/basic.vtt");
        let result = sniff_format(&file);

        assert_eq!(result.unwrap(), SubFormat::VTT);
    }

    #[test]
    pub fn test_sniff_err() {
        let file = util::read_file("tests/data/basic.non");
        assert_eq!(sniff_format(&file), Err(FormatError::UnsupportedFormat));
    }
}
