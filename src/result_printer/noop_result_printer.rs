use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::line_counter::LineCount;
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};
use encoding_rs::Encoding;
use std::path::Path;
use std::time::Duration;

pub struct NoopResultPrinter {}

impl ResultPrinter for NoopResultPrinter {
    fn set_options(&mut self, _options: &FinalDisplayOptions) {}

    fn print_result(&self, total: WalkPathResult, _time: &Duration) {
        println!("{}", total.line_count);
    }

    fn print_subtotal(&self, total: LineCount) {
        println!(": {}", total);
    }

    fn print_folder_total(&self, _total: LineCount, _depth: i32) {}

    fn print_header(&self, path: &Path, _num_entries: usize) {
        println!("{}", path.display());
    }

    fn print_folder(&self, _entry: &PrinterEntry, _num_entries: usize, _depth: i32) {}

    fn print_file(
        &self,
        _entry: &PrinterEntry,
        _lines: LineCount,
        _process_time: i64,
        _encoding: &'static Encoding,
        _depth: i32,
        _confidence: f32,
    ) {
    }

    fn print_empty_file(
        &self,
        _entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        _depth: i32,
        _confidence: f32,
    ) {
    }

    fn print_error_file(
        &self,
        _entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        _depth: i32,
        _confidence: f32,
    ) {
    }

    fn requires_advanced_walker(&self) -> bool {
        false
    }
}
