pub mod walk_path_result;

use std::collections::HashSet;
use std::io::Error;
use std::path::Path;
use encoding_rs::Encoding;
use walkdir::WalkDir;
use walk_path_result::WalkPathResult;
use crate::line_counter::{count_lines, detect_encoding};
use crate::result_printer::ResultPrinter;

pub struct ExcludeOptions<'a> {
    pub include_extensions: &'a HashSet<String>,
    pub exclude: &'a HashSet<String>,
}

pub fn walk_path(
    path: &Path,
    encoding: Option<&'static Encoding>,
    depth: i32,
    printer: &impl ResultPrinter,
    exclude_options: &ExcludeOptions,
) -> Result<WalkPathResult, Error> {
    let mut walk_result = WalkPathResult::new();
    walk_result.folder_count = 1;
    let skip_name_check = exclude_options.exclude.len() == 0;
    let skip_ext_check = exclude_options.include_extensions.len() == 0;

    let entries = WalkDir::new(path).min_depth(1).max_depth(1).into_iter().count();

    if depth == 0 {
        printer.print_header(path.to_str().unwrap(), entries);
    } else {
        printer.print_folder(&path.file_name().unwrap().to_os_string().into_string().unwrap(), entries, depth + 1);
    }

    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();
                let file_name = entry_path.file_name().unwrap().to_os_string().into_string().unwrap();
                let file_ext = entry_path.extension();

                if (skip_name_check || !exclude_options.exclude.contains(&file_name))
                    && (file_ext.is_none()
                    || skip_ext_check
                    || exclude_options.include_extensions.contains(&*file_ext.unwrap().to_os_string().into_string().unwrap()))
                {
                    if dir_entry.file_type().is_file() {
                        let used_encoding: &'static Encoding = match encoding {
                            None => {
                                let detected = detect_encoding(entry_path);
                                if detected.is_err() { return Err(detected.err().unwrap()); }
                                detected.unwrap()
                            }
                            Some(e) => e
                        };
                        match count_lines(entry_path, used_encoding) {
                            Ok(lines) => {
                                walk_result.line_count += lines as i64;
                                if lines == 0 {
                                    printer.print_empty_file(&file_name, -1, used_encoding, depth);
                                    walk_result.empty_file_count += 1;
                                } else {
                                    printer.print_file(&file_name, lines as i64, -1, used_encoding, depth);
                                    walk_result.file_count += 1;
                                }
                            }
                            Err(_) => {
                                printer.print_error_file(&file_name, -1, used_encoding, depth);
                                walk_result.error_file_count += 1;
                            }
                        }
                    } else {
                        match walk_path(entry_path, encoding, depth + 1, printer, exclude_options) {
                            Ok(sub_res) => {
                                printer.print_folder_total(sub_res.line_count, depth + 1);
                                walk_result += sub_res;
                            }
                            Err(err) =>  return Err(err)
                        }
                    }
                }
            }
            Err(err) => return Err(Error::from(err))
        }
    }

    return Ok(walk_result);
}