use std::cmp::max;
use std::path::Path;
use std::time::Duration;

use ansi_term::ANSIGenericString;
use ansi_term::Color::{Green, Purple, Red, White, Yellow};
use encoding_rs::Encoding;
use num_format::{Locale, ToFormattedString};

use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::line_counter::{LineCount, LineCountFormat};
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};

pub struct VerboseResultPrinter {
    options: FinalDisplayOptions,
}

impl VerboseResultPrinter {
    pub fn new() -> VerboseResultPrinter {
        VerboseResultPrinter {
            options: FinalDisplayOptions {
                show_all: false,
                verbose: false,
                very_verbose: false,
                simple: false,
                line_count_format: LineCountFormat::Colour { show_bytes: true },
            },
        }
    }
}

fn pad_ended(depth: i32, end: &str) -> ANSIGenericString<'_, str> {
    White
        .dimmed()
        .paint("│ ".repeat(max(depth, 0) as usize) + end)
}

impl ResultPrinter for VerboseResultPrinter {
    fn set_options(&mut self, options: &FinalDisplayOptions) {
        self.options = *options;
    }

    fn print_result(&self, total: WalkPathResult, time: &Duration) {
        let empty_file_str = White
            .dimmed()
            .paint(format!("{} empty", total.empty_file_count));
        let error_file_str = if total.error_file_count == 0 {
            White.dimmed()
        } else {
            Red.normal()
        }
        .paint(format!("{} invalid", total.error_file_count));
        println!(
            "{} file{} {} {} {} folder{} {:?}",
            total.total_files(),
            if total.total_files() == 1 { "" } else { "s" },
            empty_file_str,
            error_file_str,
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

    fn print_folder_total(&self, total: LineCount, depth: i32) {
        println!(
            "{} {}",
            pad_ended(depth, "└"),
            total.as_fmt_string(&self.options.line_count_format)
        );
    }

    fn print_header(&self, path: &Path, num_entries: usize) {
        println!(
            "{} :: {}",
            Purple.paint(path.display().to_string()),
            Yellow.paint(num_entries.to_formatted_string(&Locale::en_GB))
        );
    }

    fn print_folder(&self, entry: &PrinterEntry, num_entries: usize, depth: i32) {
        println!(
            "{}{} :: {}",
            pad_ended(depth, "├"),
            Purple.paint(&entry.name),
            Yellow.paint(num_entries.to_formatted_string(&Locale::en_GB))
        )
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
        let verbose_info = if self.options.very_verbose {
            format!(
                " [{}{}]",
                encoding.name(),
                if confidence == -1f32 {
                    "".to_string()
                } else {
                    format!(" {:.2}%", confidence * 100f32)
                }
            )
        } else {
            "".to_string()
        };
        println!(
            "{}{} :: {}{}",
            pad_ended(depth, "├"),
            Green.paint(&entry.name),
            lines.as_fmt_string(&self.options.line_count_format),
            White.dimmed().paint(verbose_info)
        );
    }

    fn print_empty_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        depth: i32,
        _confidence: f32,
    ) {
        if self.options.show_all {
            println!(
                "{}{} :: {}",
                pad_ended(depth, "├"),
                Green.paint(&entry.name),
                White.dimmed().paint("EMPTY")
            );
        }
    }

    fn print_error_file(
        &self,
        entry: &PrinterEntry,
        _process_time: i64,
        _encoding: &'static Encoding,
        depth: i32,
        _confidence: f32,
    ) {
        if self.options.show_all {
            println!(
                "{}{} :: {}",
                pad_ended(depth, "├"),
                Green.paint(&entry.name),
                Red.paint("ERROR")
            );
        }
    }

    fn requires_advanced_walker(&self) -> bool {
        true
    }
}
