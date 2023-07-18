use std::path::Path;
use std::time::Duration;
use encoding_rs::Encoding;
use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};

pub struct SimpleResultPrinter {
    options: FinalDisplayOptions
}

impl SimpleResultPrinter {
    pub fn new() -> SimpleResultPrinter {
        SimpleResultPrinter {
            options: FinalDisplayOptions {
                show_all: false,
                verbose: false,
                very_verbose: false,
                simple: false,
            },
        }
    }
}

impl ResultPrinter for SimpleResultPrinter {
    fn set_options(&mut self, options: &FinalDisplayOptions) {
        self.options = options.clone();
    }

    fn print_result(&self, total: WalkPathResult, time: &Duration) {
        println!("{} file{} {} empty {} invalid {} folder{} {:?}", total.total_files(), if total.total_files() == 1 {""} else {"s"}, total.empty_file_count, total.error_file_count, total.folder_count, if total.folder_count == 1 { "" } else { "s" }, time);
        println!("{} lines", total.line_count);
    }

    fn print_subtotal(&self, total: i64) {
        println!(": {} lines", total);
    }

    fn print_folder_total(&self, _total: i64, _depth: i32) {}

    fn print_header(&self, path: &Path, num_entries: usize) {
        println!("{} :: {num_entries}", path.display());
    }

    fn print_folder(&self, entry: &PrinterEntry, num_entries: usize, _depth: i32) {
        println!("{} :: {num_entries}", entry.path.display())
    }

    fn print_file(&self, entry: &PrinterEntry, lines: i64, _process_time: i64, encoding: &'static Encoding, _depth: i32, confidence: f32) {
        if self.options.very_verbose {
            println!("{} :: {lines} [{}{}]", entry.path.display(), encoding.name(), if confidence == -1f32 {"".to_string()} else { format!(" {:.2}%", confidence * 100f32) })
        } else {
            println!("{} :: {lines}", entry.path.display())
        }
    }

    fn print_empty_file(&self, entry: &PrinterEntry, _process_time: i64, _encoding: &'static Encoding, _depth: i32, _confidence: f32) {
        if self.options.show_all {
            println!("{} :: EMPTY", entry.path.display())
        }
    }

    fn print_error_file(&self, entry: &PrinterEntry, _process_time: i64, _encoding: &'static Encoding, _depth: i32, _confidence: f32) {
        if self.options.show_all {
            println!("{} :: ERROR", entry.path.display())
        }
    }

    fn requires_advanced_walker(&self) -> bool {
        return false;
    }
}