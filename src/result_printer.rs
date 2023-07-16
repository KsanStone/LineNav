pub mod debug_result_printer;
pub mod simple_result_printer;
pub mod noop_result_printer;
pub mod verbose_result_printer;

use std::path::Path;
use encoding_rs::Encoding;
use crate::counter_walker::walk_path_result::WalkPathResult;

#[derive(Debug, Copy, Clone)]
pub struct FinalDisplayOptions {
    pub show_all: bool,
    pub verbose: bool,
    pub very_verbose: bool,
    pub simple: bool,
}

#[derive(Debug)]
pub struct PrinterEntry<'a> {
    pub name: String,
    pub path: &'a Path
}

impl PrinterEntry<'_> {
    pub fn from_path(path: &Path) -> PrinterEntry {
        let name= path.file_name().unwrap().to_os_string().into_string().unwrap();
        PrinterEntry { name , path: &path }
    }
}

pub trait ResultPrinter {
    fn set_options(&mut self, options: &FinalDisplayOptions);

    fn print_result(&self, total: WalkPathResult, time: i64);
    fn print_subtotal(&self, total: i64);
    fn print_folder_total(&self, total: i64, depth: i32);
    fn print_header(&self, path: &Path, num_entries: usize);

    fn print_folder(&self, entry: &PrinterEntry, num_entries: usize, depth: i32);
    fn print_file(&self, entry: &PrinterEntry, lines: i64, process_time: i64, encoding: &'static Encoding, depth: i32, confidence: f32);
    fn print_empty_file(&self, entry: &PrinterEntry, process_time: i64, encoding: &'static Encoding, depth: i32, confidence: f32);
    fn print_error_file(&self, entry: &PrinterEntry, process_time: i64, encoding: &'static Encoding, depth: i32, confidence: f32);

    fn requires_advanced_walker(&self) -> bool;
}