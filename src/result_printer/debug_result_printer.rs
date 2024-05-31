use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::line_counter::LineCount;
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};
use encoding_rs::Encoding;
use std::path::Path;
use std::time::Duration;

pub struct DebugResultPrinter {}

impl ResultPrinter for DebugResultPrinter {
    fn set_options(&mut self, options: &FinalDisplayOptions) {
        println!("op set {options:?}");
    }

    fn print_result(&self, total: WalkPathResult, time: &Duration) {
        println!("total: {total:?}, {time:?}");
    }

    fn print_subtotal(&self, total: LineCount) {
        println!("sub-total: {total:?}");
    }

    fn print_folder_total(&self, total: LineCount, depth: i32) {
        println!("{depth} folder total: {total:?}");
    }

    fn print_header(&self, path: &Path, num_entries: usize) {
        println!("{} :: {num_entries}", path.to_str().unwrap());
    }

    fn print_folder(&self, entry: &PrinterEntry, num_entries: usize, depth: i32) {
        println!("{depth} folder: {} :: {num_entries}", entry.name);
    }

    fn print_file(
        &self,
        entry: &PrinterEntry,
        lines: LineCount,
        _process_time: i64,
        encoding: &'static Encoding,
        depth: i32,
        confidence: f32,
    ) {
        println!(
            "{depth} file: {} {lines:?} {encoding:?}[{confidence}]",
            entry.name
        );
    }

    fn print_empty_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        encoding: &'static Encoding,
        depth: i32,
        confidence: f32,
    ) {
        println!("{depth} empty: {} {encoding:?}[{confidence}]", entry.name);
    }

    fn print_error_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        encoding: &'static Encoding,
        depth: i32,
        confidence: f32,
    ) {
        println!("{depth} error: {} {encoding:?}[{confidence}]", entry.name);
    }

    fn requires_advanced_walker(&self) -> bool {
        true
    }
}
