mod line_counter;
mod counter_walker;
mod result_printer;

use std::{fs, process};
use std::collections::HashSet;
use std::path::PathBuf;
use clap::{Parser};
use encoding_rs::Encoding;
use crate::counter_walker::{ExcludeOptions, walk_path};
use crate::counter_walker::walk_path_result::WalkPathResult;
use crate::result_printer::{FinalDisplayOptions, ResultPrinter};
use crate::result_printer::debug_result_printer::DebugResultPrinter;
use crate::result_printer::simple_result_printer::SimpleResultPrinter;

#[derive(Debug, Parser)]
#[command(name = "CMDStore")]
/// Key-value based storage utility for the commandline
struct LineNavArgs {
    #[clap(long, short, action)]
    verbose: bool,
    #[clap(long = "vv", action)]
    very_verbose: bool,
    #[clap(long, short, action)]
    all_files: bool,
    #[clap(long, short, action)]
    simple: bool,
    #[clap(long, short, default_value = "UTF-8")]
    encoding: String,
    #[clap(long, short = 'f', required = false)]
    file_extensions: Option<String>,
    #[clap(num_args = 0.., default_values = ["."])]
    paths: Vec<String>,
    #[clap(long, short = 'x', num_args = 1.., required = false)]
    exclude: Vec<String>,
}

fn main() {
    let args = LineNavArgs::parse();
    let include_extensions: HashSet<String> = match args.file_extensions {
        None => HashSet::new(),
        Some(ref extensions) => extensions.split(',').map(str::to_string).collect()
    };
    let exclude: HashSet<String> = args.exclude.iter().map(|x| x.to_owned()).collect();
    let paths: Vec<PathBuf> = args.paths.iter().map(fs::canonicalize).map(|x| {
        match x {
            Ok(path) => path,
            Err(err) => {
                eprintln!("Invalid path. {err:?}");
                process::exit(1);
            }
        }
    }).collect();

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
        verbose: args.verbose,
        very_verbose: args.very_verbose,
        simple: args.simple,
    };

    println!("{args:?} {encoding:?} {include_extensions:?}");

    let mut printer_impl: Box<dyn ResultPrinter> = if display_options.verbose && display_options.simple {
        println!("simple impl");
        Box::new(SimpleResultPrinter::new())
    } else {
        println!("debug impl");
        Box::new(DebugResultPrinter {})
    };

    (&mut *printer_impl).set_options(&display_options);

    let mut final_res = WalkPathResult::new();

    for path in paths.iter() {
        let sub_count = walk_path(path, encoding, 0, &*printer_impl, &ExcludeOptions { include_extensions: &include_extensions, exclude: &exclude }).expect("Count failed");
        if paths.len() > 1 {
            printer_impl.print_subtotal(sub_count.line_count.clone());
        }
        final_res += sub_count;
    }

    printer_impl.print_result(final_res);
}
