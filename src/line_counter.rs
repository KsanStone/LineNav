use ansi_term::Color::{Blue, White};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::{Add, AddAssign};
use std::path::Path;

use chardet::detect;
use encoding_rs::{Encoding, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;
use humansize::{format_size, WINDOWS};
use num_format::{Locale, ToFormattedString};

#[derive(Clone, Copy, Debug)]
pub enum LineCountFormat {
    Simple { show_bytes: bool },
    Colour { show_bytes: bool },
}

#[derive(Clone, Copy, Debug)]
pub struct LineCount {
    pub lines: u64,
    pub bytes: u64,
}

impl fmt::Display for LineCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_simple(false))
    }
}

impl LineCount {
    pub fn new() -> LineCount {
        LineCount { lines: 0, bytes: 0 }
    }

    fn as_simple(&self, show_bytes: bool) -> String {
        let loc_formatted = self.lines.to_string();
        if show_bytes {
            let bytes_formatted = format_size(self.bytes, WINDOWS);
            format!("{}loc {}", loc_formatted, bytes_formatted)
        } else {
            loc_formatted
        }
    }

    fn as_colour(&self, show_bytes: bool) -> String {
        let loc_formatted = self.lines.to_formatted_string(&Locale::en);
        if show_bytes {
            let bytes_formatted = format_size(self.bytes, WINDOWS);
            format!(
                "{} {}",
                Blue.paint(loc_formatted),
                White.dimmed().paint(bytes_formatted)
            )
        } else {
            loc_formatted
        }
    }

    pub fn as_fmt_string(&self, display_format: &LineCountFormat) -> String {
        match display_format {
            LineCountFormat::Simple { show_bytes } => self.as_simple(show_bytes.to_owned()),
            LineCountFormat::Colour { show_bytes } => self.as_colour(show_bytes.to_owned()),
        }
    }
}

impl Add for LineCount {
    type Output = LineCount;

    fn add(self, rhs: Self) -> Self::Output {
        LineCount {
            lines: self.lines + rhs.lines,
            bytes: self.bytes + rhs.bytes,
        }
    }
}

impl AddAssign for LineCount {
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.bytes += rhs.bytes;
    }
}

pub fn count_lines(file: &Path, encoding: &'static Encoding) -> Result<LineCount, Error> {
    match File::open(file) {
        Ok(fp) => {
            let bytes = fp.metadata().unwrap().len();
            let reader = BufReader::new(
                DecodeReaderBytesBuilder::new()
                    .encoding(Some(encoding))
                    .build(fp),
            );
            let mut count = 0;
            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        if line.contains('\u{FFFD}') {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Line count failed whilst using encoding {encoding:?}"),
                            ));
                        }
                        count += 1
                    }
                    Err(err) => return Err(err),
                }
            }
            Ok(LineCount {
                lines: count,
                bytes,
            })
        }
        Err(err) => Err(err),
    }
}

pub struct DetectedEncoding {
    pub encoding: &'static Encoding,
    pub confidence: f32,
}

pub fn detect_encoding(file: &Path) -> Result<DetectedEncoding, Error> {
    match OpenOptions::new().read(true).open(file) {
        Ok(fh) => {
            let mut reader: Vec<u8> = Vec::new();
            let mut chunk = fh.take(8192);
            let _read_result = chunk.read_to_end(&mut reader)?;

            let result = detect(&reader);
            Ok(DetectedEncoding {
                encoding: Encoding::for_label(result.0.as_bytes()).unwrap_or(UTF_8),
                confidence: result.1,
            })
        }
        Err(err) => Err(err),
    }
}
