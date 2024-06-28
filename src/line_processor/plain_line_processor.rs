use crate::line_counter::LineCount;
use crate::line_processor::LineProcessor;
use encoding_rs::Encoding;
use std::io::{Error, ErrorKind};

pub struct PlainLineProcessor {}

impl LineProcessor for PlainLineProcessor {
    fn process_line(
        &mut self,
        line: &str,
        encoding: &'static Encoding,
    ) -> Result<LineCount, Error> {
        if line.contains('\u{FFFD}') {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Line count failed whilst using encoding {encoding:?}"),
            ));
        }
        if line.chars().all(char::is_whitespace) {
            Ok(LineCount {
                lines: 1,
                blank_lines: 1,
                bytes: 0,
            })
        } else {
            Ok(LineCount {
                lines: 1,
                blank_lines: 0,
                bytes: 0,
            })
        }
    }
}
