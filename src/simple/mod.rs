use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Result, Seek};
use std::io::SeekFrom::Start;

const BUF_SIZE: usize = 4096;

// -----------------------------------------------------------------------------
//     - Owned entry -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Entry {
    data: Vec<u8>,
    line_no: usize, // id
}

impl Entry {
    pub fn new(line_no: usize, data: Vec<u8>) -> Self {
        Self { data, line_no }
    }
}

// -----------------------------------------------------------------------------
//     - Table -
// -----------------------------------------------------------------------------
pub struct Table {
    reader: BufReader<File>,
    file: File,
}

impl Table {
    pub fn new(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;

        let reader = BufReader::new(file);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)?;


        let inst = Self { reader, file };

        Ok(inst)
    }

    pub fn find(&mut self, kw: &[u8]) -> Vec<Entry> {
        if kw.len() == 0 {
            return Vec::new();
        }

        let mut results = Vec::new();
        let mut buf = String::new();
        let mut line_offset = 0;
        let needle = std::str::from_utf8(kw).unwrap();

        while let Ok(bytes_read) = self.reader.read_line(&mut buf) {
            if bytes_read == 0 {
                break;
            }
            line_offset += 1;

            if buf.contains(needle) {
                let bytes = buf.as_bytes().to_vec();
                let ent = Entry::new(line_offset, bytes);
                results.push(ent);
            }

            buf.clear();
        }

        self.reader.seek(Start(0));

        results
    }

    pub fn delete(&mut self, entry: Entry) -> Option<()> {
    //     // entry.line_no

    //     let mut buf = [0;4096];
    //     let mut offset = 0;

    //     loop {
    //         if let Ok(n) = self.reader.read(&mut buf) {
    //             if n == 0 {
    //                 break
    //             }

    //             let mut data: &[u8] = &buf;

    //             loop {
    //                 if let Some(pos) = data.iter().position(|b| b == &b'\n') {
    //                     offset += pos;
    //                     data = &data[pos..];
    //                 } else {
    //                     break
    //                 }
    //             }
    //         }
    //     }

        None
    }

    pub fn update(&mut self, entry: Entry, data: &[u8]) {
        self.delete(entry);
        self.insert(data);
    }

    pub fn insert(&mut self, data: &[u8]) {
        if data.len() > BUF_SIZE {
            panic!("Data is massive mate!");
        }

        self.file.write(data);
        self.file.write(&[b'\n']);
        self.file.flush();
    }
}
