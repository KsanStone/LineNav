mod plain_line_processor;

use crate::line_counter::LineCount;
use std::ffi::OsStr;
use std::io::Error;
use encoding_rs::Encoding;
use crate::line_processor::plain_line_processor::PlainLineProcessor;

pub struct LineProcessorFactory {}

pub trait LineProcessor {
    fn process_line(&mut self, line: &str, encoding: &'static Encoding) -> Result<LineCount, Error>;
}

impl LineProcessorFactory {
    pub fn create(_: &OsStr) -> Box<dyn LineProcessor> {
        Box::new(PlainLineProcessor {})
    }
}
