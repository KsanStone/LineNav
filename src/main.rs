use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Instant;
use std::{fs, process};

use clap::{ArgAction, Parser};
use encoding_rs::Encoding;

use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::counter_walker::{handle_file_entry, simple_walk_path, walk_path, ExcludeOptions};
use crate::line_counter::LineCountFormat;
use crate::result_printer::debug_result_printer::DebugResultPrinter;
use crate::result_printer::noop_result_printer::NoopResultPrinter;
use crate::result_printer::simple_result_printer::SimpleResultPrinter;
use crate::result_printer::verbose_result_printer::VerboseResultPrinter;
use crate::result_printer::{FinalDisplayOptions, PrinterEntry, ResultPrinter};
use crate::summarizer::default_summarizer::DefaultSummarizer;
use crate::summarizer::leaderboard_summarizer::LeaderboardSummarizer;
use crate::summarizer::noop_summarizer::NoopSummarizer;
use crate::summarizer::Summarizer;

mod counter_walker;
mod line_counter;
mod line_processor;
mod result_printer;
mod summarizer;

#[derive(Debug, Parser)]
#[command(name = "CMDStore")]
/// Project line counter utility
struct LineNavArgs {
    #[arg(long, short, action = ArgAction::Count)]
    /// Shows a tree with all the files
    /// Add a second flag to enable --very-verbose output
    verbose: u8,
    #[clap(long, action)]
    /// Shows the encoding
    very_verbose: bool,
    #[clap(long, short, action)]
    /// Shows empty and invalid files
    all_files: bool,
    #[clap(long, short, action)]
    /// Simplified console output
    simple: bool,
    #[clap(long, short, action)]
    debug: bool,
    #[clap(long, short, default_value = "UTF-8")]
    /// Encoding to read files with. Set "auto" to automatically detect
    encoding: String,
    #[clap(long, short = 'f', required = false, value_delimiter = ',')]
    /// File extensions to count through
    file_extensions: Vec<String>,
    #[clap(num_args = 0.., default_values = ["."])]
    /// Folders to count
    paths: Vec<String>,
    #[clap(long, short = 'x', num_args = 1.., required = false)]
    /// Excluded file names
    exclude: Vec<String>,
    #[clap(long, short = 'm', default_missing_value = "0", num_args = 0..)]
    /// Summarize line counts by file extension
    summary: Option<u32>,
    /// Display a leaderboard of longest files
    #[clap(long, short, action)]
    leaderboard: bool,
}

fn main() {
    let args = LineNavArgs::parse();
    let include_extensions: HashSet<String> =
        args.file_extensions.iter().map(|x| x.to_owned()).collect();
    let exclude: HashSet<String> = args.exclude.iter().map(|x| x.to_owned()).collect();
    let paths: Vec<PathBuf> = args
        .paths
        .iter()
        .map(fs::canonicalize)
        .map(|x| match x {
            Ok(path) => path,
            Err(err) => {
                eprintln!("Invalid path. {err:?}");
                process::exit(1);
            }
        })
        .collect();

    #[cfg(target_os = "windows")]
    {
        let _ = ansi_term::enable_ansi_support();
    }

    let encoding: Option<&'static Encoding> = if args.encoding == "auto" {
        None
    } else {
        let encoding = Encoding::for_label(args.encoding.as_bytes());
        if encoding.is_none() {
            eprintln!("Invalid encoding");
            process::exit(1);
        }
        Some(encoding.unwrap())
    };

    let display_options = FinalDisplayOptions {
        show_all: args.all_files,
        verbose: args.verbose > 0,
        very_verbose: args.very_verbose || args.verbose > 1,
        simple: args.simple,
        line_count_format: if args.very_verbose || args.verbose > 0 {
            LineCountFormat::Colour { show_bytes: true }
        } else {
            LineCountFormat::Simple { show_bytes: false }
        },
    };

    let mut printer_impl: Box<dyn ResultPrinter> =
        if display_options.verbose && display_options.simple {
            Box::new(SimpleResultPrinter::new())
        } else if display_options.verbose {
            Box::new(VerboseResultPrinter::new())
        } else if args.debug {
            Box::new(DebugResultPrinter {})
        } else {
            Box::new(NoopResultPrinter {})
        };

    (*printer_impl).set_options(&display_options);

    let mut summarizer: Box<dyn Summarizer> = if args.summary.is_some() {
        Box::new(DefaultSummarizer::new())
    } else if args.leaderboard {
        Box::new(LeaderboardSummarizer::new())
    } else {
        Box::new(NoopSummarizer::new())
    };

    let mut final_res = WalkPathResult::new();
    let start = Instant::now();

    for path in paths.iter() {
        if path.is_dir() {
            let sub_count = if printer_impl.requires_advanced_walker() {
                walk_path(
                    path,
                    encoding,
                    0,
                    &*printer_impl,
                    &mut *summarizer,
                    &ExcludeOptions {
                        include_extensions: &include_extensions,
                        exclude: &exclude,
                    },
                )
                .expect("Count failed")
            } else {
                simple_walk_path(
                    path,
                    encoding,
                    &*printer_impl,
                    &mut *summarizer,
                    &ExcludeOptions {
                        include_extensions: &include_extensions,
                        exclude: &exclude,
                    },
                )
                .expect("Count failed")
            };
            if paths.len() > 1 {
                printer_impl.print_subtotal(sub_count.line_count);
            }
            final_res += sub_count;
        } else if path.is_file() {
            let res = &mut WalkPathResult::new();
            handle_file_entry(
                encoding,
                path,
                &PrinterEntry::from_path(path),
                0,
                res,
                &NoopResultPrinter {},
                &mut *summarizer,
            )
            .expect("Count failed");
            printer_impl.print_header(path, 1);
            printer_impl.print_subtotal(res.line_count);
            final_res += *res;
        }
    }

    let duration = start.elapsed();

    if args.summary.is_some() {
        summarizer.set_limit(args.summary.unwrap());
        summarizer.print_summary(final_res.line_count);
    } else if args.leaderboard {
        summarizer.print_summary(final_res.line_count);
    }

    printer_impl.print_result(final_res, &duration);
}
