use crate::Hash;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::str;

#[derive(Debug)]
pub struct SourceReader {
    reader: BufReader<File>,
}

///
/// The reader of input file with one block hash per line.
///
/// Utilizes the BufReader internally to amortize the time cost for disk
/// operations.
///
/// Assumptions:
/// - A hash is a 64 bytes long ASCII string
/// - A hash is a base16 string
/// - A hash is a lowercase string
/// - Hashes are separated by newlines ('\n')
///
/// Implements Iterator trait <...>
///
/// # Examples
/// <...>
#[allow(dead_code)]
impl SourceReader {
    ///
    /// Creates a new input file reader with the BufReader of default buffer
    /// size. To tune the buffer size, use [`with_buffer_capacity`].
    pub fn new(filename: String) -> Result<Self, Error> {
        let file = File::open(filename)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    ///
    /// Creates a new input file reader with the BufReader of specified buffer
    /// size.
    pub fn with_buffer_capacity(capacity: usize, filename: String) -> Result<Self, Error> {
        let file = File::open(filename)?;
        Ok(Self {
            reader: BufReader::with_capacity(capacity, file),
        })
    }
}

impl Iterator for SourceReader {
    type Item = Hash;
    fn next(&mut self) -> Option<Self::Item> {
        let mut str_buf = [0u8; 65]; // 65: accomodate for newline
        let mut byte_buf = [0u8; 32];

        // This code may actually work wrong if the last hash is corrupted, e.g.
        // too short (63 bytes instead of 64). In this case last hash will be
        // silently ignored.
        if let Ok(()) = self.reader.read_exact(&mut str_buf) {
            let str = str::from_utf8(&str_buf[0..64]).expect("Expected valid UTF-8 string");
            base16ct::lower::decode(str, &mut byte_buf).expect("Expected valid base16 string");
            return Some(byte_buf);
        }
        None
    }
}
