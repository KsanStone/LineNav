pub mod debug_result_printer;

use encoding_rs::Encoding;
use crate::counter_walker::walk_path_result::WalkPathResult;

#[derive(Debug)]
pub struct FinalDisplayOptions {
    pub show_all: bool,
    pub verbose: bool,
    pub very_verbose: bool,
    pub simple: bool,
}

pub trait ResultPrinter {
    fn set_options(&self, options: &FinalDisplayOptions);

    fn print_total(&self, total: WalkPathResult);
    fn print_subtotal(&self, total: i64);
    fn print_folder_total(&self, total: i64, depth: i32);
    fn print_header(&self, path: &str, num_entries: usize);

    fn print_folder(&self, name: &String, num_entries: usize, depth: i32);
    fn print_file(&self, name: &String, lines: i64, process_time: i64, encoding: &'static Encoding, depth: i32);
    fn print_empty_file(&self, name: &String, process_time: i64, encoding: &'static Encoding, depth: i32);
    fn print_error_file(&self, name: &String, process_time: i64, encoding: &'static Encoding, depth: i32);
}