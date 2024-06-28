use crate::line_counter::LineCount;
use crate::summarizer::Summarizer;
use std::path::Path;

pub struct NoopSummarizer {}

impl NoopSummarizer {
    pub fn new() -> Self {
        NoopSummarizer {}
    }
}

impl Summarizer for NoopSummarizer {
    fn append_entry(&mut self, _file_path: &Path, _entry: LineCount) {}

    fn set_limit(&mut self, _limit: u32) {}

    fn print_summary(&self) {}
}
