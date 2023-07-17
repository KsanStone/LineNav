pub mod walk_path_result;

use std::collections::HashSet;
use std::io::Error;
use std::path::Path;
use encoding_rs::Encoding;
use walkdir::WalkDir;
use walk_path_result::WalkPathResult;
use crate::line_counter::{count_lines, detect_encoding};
use crate::result_printer::{PrinterEntry, ResultPrinter};

pub struct ExcludeOptions<'a> {
    pub include_extensions: &'a HashSet<String>,
    pub exclude: &'a HashSet<String>,
}

fn handle_file_entry(
    encoding: Option<&'static Encoding>,
    entry_path: &Path,
    entry: &PrinterEntry,
    depth: i32,
    walk_result: &mut WalkPathResult,
    printer: &(impl ResultPrinter + ?Sized),
) -> Result<(), Error> {
    let mut confidence = -1f32;
    let used_encoding: &'static Encoding = match encoding {
        None => {
            let detected = detect_encoding(entry_path);
            if detected.is_err() { return Err(detected.err().unwrap()); }
            let detected_encoding = detected.unwrap();
            confidence = detected_encoding.confidence;
            detected_encoding.encoding
        }
        Some(e) => e
    };
    match count_lines(entry_path, used_encoding) {
        Ok(lines) => {
            walk_result.line_count += lines as i64;
            if lines == 0 {
                printer.print_empty_file(&entry, -1, used_encoding, depth, confidence);
                walk_result.empty_file_count += 1;
            } else {
                printer.print_file(&entry, lines as i64, -1, used_encoding, depth, confidence);
                walk_result.file_count += 1;
            };
            Ok(())
        }
        Err(err) => {
            printer.print_error_file(&entry, -1, used_encoding, depth, confidence);
            walk_result.error_file_count += 1;
            Ok(())
        }
    }
}

pub fn walk_path(
    path: &Path,
    encoding: Option<&'static Encoding>,
    depth: i32,
    printer: &(impl ResultPrinter + ?Sized),
    exclude_options: &ExcludeOptions,
) -> Result<WalkPathResult, Error> {
    let mut walk_result = WalkPathResult::new();
    walk_result.folder_count = 1;
    let skip_name_check = exclude_options.exclude.len() == 0;
    let skip_ext_check = exclude_options.include_extensions.len() == 0;

    let entries = WalkDir::new(path).min_depth(1).max_depth(1).into_iter().count();

    if depth == 0 {
        printer.print_header(path, entries);
    } else {
        printer.print_folder(&PrinterEntry::from_path(path), entries, depth - 1);
    }

    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();
                let file_ext = entry_path.extension();
                let entry = PrinterEntry { name: entry_path.file_name().unwrap().to_os_string().into_string().unwrap(), path: &entry_path };

                if (skip_name_check || !exclude_options.exclude.contains(&*entry.name))
                    && (file_ext.is_none()
                    || skip_ext_check
                    || exclude_options.include_extensions.contains(&*file_ext.unwrap().to_os_string().into_string().unwrap()))
                {
                    if dir_entry.file_type().is_file() {
                        if let Err(res) = handle_file_entry(
                            encoding,
                            entry_path,
                            &entry,
                            depth,
                            &mut walk_result,
                            printer
                        ) {
                            return Err(res);
                        }
                    } else {
                        match walk_path(entry_path, encoding, depth + 1, printer, exclude_options) {
                            Ok(sub_res) => {
                                printer.print_folder_total(sub_res.line_count, depth + 1);
                                walk_result += sub_res;
                            }
                            Err(err) => return Err(err)
                        }
                    }
                }
            }
            Err(err) => return Err(Error::from(err))
        }
    }

    return Ok(walk_result);
}

pub fn simple_walk_path(
    path: &Path,
    encoding: Option<&'static Encoding>,
    printer: &(impl ResultPrinter + ?Sized),
    exclude_options: &ExcludeOptions,
) -> Result<WalkPathResult, Error> {
    let mut walk_result = WalkPathResult::new();
    walk_result.folder_count = 1;
    let skip_name_check = exclude_options.exclude.len() == 0;
    let skip_ext_check = exclude_options.include_extensions.len() == 0;

    let entries = WalkDir::new(path).min_depth(1).max_depth(1).into_iter().count();

    printer.print_header(path, entries);

    for entry in WalkDir::new(path).min_depth(1) {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();
                let file_ext = entry_path.extension();
                let entry = PrinterEntry { name: entry_path.file_name().unwrap().to_os_string().into_string().unwrap(), path: &entry_path };

                if (skip_name_check || !exclude_options.exclude.contains(&*entry.name))
                    && (file_ext.is_none()
                    || skip_ext_check
                    || exclude_options.include_extensions.contains(&*file_ext.unwrap().to_os_string().into_string().unwrap()))
                {
                    if dir_entry.file_type().is_file() {
                        if let Err(res) = handle_file_entry(
                            encoding,
                            entry_path,
                            &entry,
                            -1,
                            &mut walk_result,
                            printer
                        ) {
                            return Err(res);
                        }
                    }
                }
            }
            Err(err) => return Err(Error::from(err))
        }
    }

    return Ok(walk_result);
}