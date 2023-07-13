use encoding_rs::Encoding;
use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::result_printer::{FinalDisplayOptions, ResultPrinter};

pub struct DebugResultPrinter {}

impl ResultPrinter for DebugResultPrinter {
    fn set_options(&self, options: &FinalDisplayOptions) {
        println!("op set {options:?}");
    }

    fn print_total(&self, total: WalkPathResult) {
        println!("total: {total:?}");
    }

    fn print_subtotal(&self, total: i64) {
        println!("sub-total: {total}");
    }

    fn print_folder_total(&self, total: i64, depth: i32) {
        println!("{depth} folder total: {total}");
    }

    fn print_header(&self, path: &str, num_entries: usize) {
        println!("{path} :: {num_entries}");
    }

    fn print_folder(&self, name: &String, num_entries: usize, depth: i32) {
        println!("{depth} folder: {name} :: {num_entries}");
    }

    fn print_file(&self, name: &String, lines: i64, _process_time: i64, encoding: &'static Encoding, depth: i32) {
        println!("{depth} file: {name} {lines} {encoding:?}");
    }

    fn print_empty_file(&self, name: &String, _process_time: i64, encoding: &'static Encoding, depth: i32) {
        println!("{depth} empty: {name} {encoding:?}");
    }

    fn print_error_file(&self, name: &String, _process_time: i64, encoding: &'static Encoding, depth: i32) {
        println!("{depth} error: {name} {encoding:?}");
    }
}