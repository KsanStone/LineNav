use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::line_counter::{LineCount, LineCountFormat};
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};
use encoding_rs::Encoding;
use std::path::Path;
use std::time::Duration;

pub struct SimpleResultPrinter {
    options: FinalDisplayOptions,
}

impl SimpleResultPrinter {
    pub fn new() -> SimpleResultPrinter {
        SimpleResultPrinter {
            options: FinalDisplayOptions {
                show_all: false,
                verbose: false,
                very_verbose: false,
                simple: false,
                line_count_format: LineCountFormat::Simple { show_bytes: false },
            },
        }
    }
}

impl ResultPrinter for SimpleResultPrinter {
    fn set_options(&mut self, options: &FinalDisplayOptions) {
        self.options = *options;
    }

    fn print_result(&self, total: WalkPathResult, time: &Duration) {
        println!(
            "{} file{} {} empty {} invalid {} folder{} {:?}",
            total.total_files(),
            if total.total_files() == 1 { "" } else { "s" },
            total.empty_file_count,
            total.error_file_count,
            total.folder_count,
            if total.folder_count == 1 { "" } else { "s" },
            time
        );
        println!(
            "{}",
            total
                .line_count
                .as_fmt_string(&self.options.line_count_format)
        );
    }

    fn print_subtotal(&self, total: LineCount) {
        println!(": {}", total.as_fmt_string(&self.options.line_count_format));
    }

    fn print_folder_total(&self, _total: LineCount, _depth: i32) {}

    fn print_header(&self, path: &Path, num_entries: usize) {
        println!("{} :: {num_entries}", path.display());
    }

    fn print_folder(&self, entry: &PrinterEntry, num_entries: usize, _depth: i32) {
        println!("{} :: {num_entries}", entry.path.display())
    }

    fn print_file(
        &self,
        entry: &PrinterEntry,
        lines: LineCount,
        _process_time: i64,
        encoding: &'static Encoding,
        _depth: i32,
        confidence: f32,
    ) {
        if self.options.very_verbose {
            println!(
                "{} :: {} [{}{}]",
                entry.path.display(),
                lines.as_fmt_string(&self.options.line_count_format),
                encoding.name(),
                if confidence == -1f32 {
                    "".to_string()
                } else {
                    format!(" {:.2}%", confidence * 100f32)
                }
            )
        } else {
            println!("{} :: {lines}", entry.path.display())
        }
    }

    fn print_empty_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        _depth: i32,
        _confidence: f32,
    ) {
        if self.options.show_all {
            println!("{} :: EMPTY", entry.path.display())
        }
    }

    fn print_error_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        _depth: i32,
        _confidence: f32,
    ) {
        if self.options.show_all {
            println!("{} :: ERROR", entry.path.display())
        }
    }

    fn requires_advanced_walker(&self) -> bool {
        false
    }
}
