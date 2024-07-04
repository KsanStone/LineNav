pub mod default_summarizer;
pub mod noop_summarizer;

use crate::line_counter::LineCount;
use std::path::Path;

pub trait Summarizer {
    fn append_entry(&mut self, file_path: &Path, entry: LineCount);

    fn set_limit(&mut self, limit: u32);

    fn print_summary(&self, total: LineCount);
}
