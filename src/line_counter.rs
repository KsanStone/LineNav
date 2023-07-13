use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::{Path};
use encoding_rs::Encoding;
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
                    Ok(_) => count += 1,
                    Err(err) => return Err(err)
                }
            }
            Ok(count)
        }
        Err(err) => Err(err),
    }
}

pub fn detect_encoding(_file: &Path) -> Result<&'static Encoding, Error> {
 todo!()
}

