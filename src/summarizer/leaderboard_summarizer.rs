use crate::line_counter::LineCount;
use crate::summarizer::Summarizer;
use prettytable::{format, row, Table};
use std::path::Path;

pub struct LeaderboardSummarizer {
    entries: Vec<(String, LineCount)>,
}

impl LeaderboardSummarizer {
    pub(crate) fn new() -> Self {
        LeaderboardSummarizer { entries: vec![] }
    }
}

impl Summarizer for LeaderboardSummarizer {
    fn append_entry(&mut self, file_path: &Path, entry: LineCount) {
        self.entries
            .push((file_path.to_string_lossy().to_string(), entry))
    }

    fn set_limit(&mut self, _limit: u32) {}

    fn print_summary(&mut self, _total: LineCount) {
        self.entries.sort_by_key(|e| e.1.lines);
        self.entries.reverse();

        let mut table = Table::new();
        table.set_titles(row!["#", "path", "lines"]);
        for entry in self.entries.iter().enumerate() {
            table.add_row(row![entry.0 + 1, entry.1 .0, entry.1 .1]);
        }

        println!();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();
    }
}
