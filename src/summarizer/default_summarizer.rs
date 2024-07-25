use ansi_term::Color::White;
use humansize::{format_size, WINDOWS};
use num_format::{Locale, ToFormattedString};
use prettytable::{format, row, Table};
use std::collections::HashMap;
use std::path::Path;

use crate::line_counter::LineCount;
use crate::summarizer::Summarizer;

/// Summarizes line counts for files grouped by their file extension
pub struct DefaultSummarizer {
    results: HashMap<String, (LineCount, u64)>,
    limit: u32,
}

impl DefaultSummarizer {
    pub fn new() -> Self {
        DefaultSummarizer {
            results: HashMap::new(),
            limit: 0,
        }
    }
}

impl Summarizer for DefaultSummarizer {
    fn append_entry(&mut self, file_path: &Path, entry: LineCount) {
        let filename = file_path.file_name().unwrap_or("".as_ref());
        let extension = file_path
            .extension()
            .unwrap_or(filename)
            .to_str()
            .unwrap()
            .to_string();

        if let std::collections::hash_map::Entry::Vacant(e) = self.results.entry(extension.clone())
        {
            e.insert((entry, 1));
        } else {
            let a = self.results.get_mut(&extension).unwrap();
            a.0 += entry;
            a.1 += 1;
        }
    }

    fn set_limit(&mut self, limit: u32) {
        self.limit = limit
    }

    fn print_summary(&self, total: LineCount) {
        let mut entries: Vec<(String, (LineCount, u64))> = self.results.clone().into_iter().collect();
        entries.sort_by(|a, b| b.1.0.lines.cmp(&a.1.0.lines));

        let mut table = Table::new();
        table.set_titles(row!["extension", "% total", "lines", "blank", "size", "entries"]);

        let mut limit = 0u32;
        for entry in &entries {
            let bytes_formatted = format_size(entry.1.0.bytes, WINDOWS);
            table.add_row(row![
                entry.0,
                format!(
                    "{:.3}%",
                    (entry.1.0.lines as f64) / (total.lines as f64) * 100f64
                ),
                entry.1.0.lines.to_formatted_string(&Locale::en_GB),
                entry.1.0.blank_lines.to_formatted_string(&Locale::en_GB),
                bytes_formatted,
                entry.1.1
            ]);
            limit += 1;
            if limit == self.limit {
                break;
            }
        }

        println!(); // Pretty padding

        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();

        if self.limit < entries.len() as u32 && self.limit != 0 {
            println!(
                "{}",
                White.dimmed().paint(format!(
                    "And {} more...",
                    entries.len() - self.limit as usize
                ))
            );
        }

        println!(); // Pretty padding
    }
}
