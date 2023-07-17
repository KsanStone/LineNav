use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::{Path};
use chardet::detect;
use encoding_rs::{Encoding, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;

pub fn count_lines(file: &Path, encoding: &'static Encoding) -> Result<usize, Error> {
    match File::open(file) {
        Ok(fp) => {
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
                            return Err(Error::new(ErrorKind::InvalidData, format!("Line count failed whilst using encoding {encoding:?}")));
                        }
                        count += 1
                    },
                    Err(err) => return Err(err)
                }
            }
            Ok(count)
        }
        Err(err) => Err(err),
    }
}

pub struct DetectedEncoding {
    pub encoding: &'static Encoding,
    pub confidence: f32,
}

pub fn detect_encoding(file: &Path) -> Result<DetectedEncoding, Error> {
    // open text file
    match OpenOptions::new().read(true).open(file) {
        Ok(fh) => {
            let mut reader: Vec<u8> = Vec::new();
            let mut chunk = fh.take(8192);
            let _read_result = chunk.read_to_end(&mut reader)?;

            let result = detect(&reader);
            Ok(DetectedEncoding { encoding: Encoding::for_label(result.0.as_bytes()).unwrap_or(UTF_8), confidence: result.1 })
        }
        Err(err) => Err(err)
    }
}

