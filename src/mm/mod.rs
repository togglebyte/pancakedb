use std::fs::{File, OpenOptions};
use std::io::Result;

use memmap::Mmap;
use netlib::memchr::memchr;

// -----------------------------------------------------------------------------
//     - Entry -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Entry<'a> {
    pub data: &'a [u8],
    offset: usize, // offset
}

impl<'a> Entry<'a> {
    pub fn new(offset: usize, data: &'a [u8]) -> Self {
        Self { data, offset }
    }
}

// -----------------------------------------------------------------------------
//     - Table -
// -----------------------------------------------------------------------------
pub struct Table {
    _file: File,
    data: Mmap,
}

fn find_nl(b: &u8) -> bool {
    b == &b'\n'
}

impl Table {
    pub fn new(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;

        let data = unsafe { Mmap::map(&file)? };

        let inst = Self { _file: file, data };

        Ok(inst)
    }

    // Look something up:
    // 1. Memmap the file
    // 2. Find first character in keyword using `netlib::memchr::memchr`
    // 3. Split the buffer on the position
    // 4. try to match the first set of bytes in the new chunk, with keyword[1..]
    // 5. Find the `\n` to the left and right of the first byte position
    pub fn find_orig(&mut self, needle: &[u8]) -> Vec<Entry> {
        let mut entries = Vec::new();

        let mut data: &[u8] = &self.data;

        while let Some(pos) = memchr(data, needle[0]) {
            let slice = &data[pos..pos + needle.len()];

            if slice == needle {
                let start = data[..pos].iter().rposition(find_nl).unwrap_or(0);
                let end = data[pos + needle.len()..].iter().position(find_nl).unwrap() + pos + needle.len();
                let entry = Entry::new(start, &data[start..end]);
                entries.push(entry);
                data = &data[end..];
            } else {
                data = &data[pos + 1..];
            }
        }

        entries
    }

    pub fn find(&mut self, needle: &[u8]) -> Vec<Entry> {
        use aho_corasick::AhoCorasickBuilder;

        let mut entries = Vec::new();

        let aho = AhoCorasickBuilder::new().build(&[needle]);
        for e in aho.find_iter(&self.data) { 
            let start = self.data[..e.start()].iter().rposition(find_nl).unwrap_or(0) + 1;
            let end = self.data[e.end()..].iter().position(find_nl).unwrap() + e.end();
            let entry = Entry::new(e.start(), &self.data[start..end]);
            entries.push(entry);
        }

        // let mut data: &[u8] = &self.data;

        // while let Some(pos) = memchr(data, needle[0]) {
        //     let slice = &data[pos..pos + needle.len()];

        //     if slice == needle {
        //         let start = data[..pos].iter().rposition(find_nl).unwrap_or(0);
        //         let end = data[pos + needle.len()..].iter().position(find_nl).unwrap() + pos + needle.len();
        //         let entry = Entry::new(start, &data[start..end]);
        //         entries.push(entry);
        //         data = &data[end..];
        //     } else {
        //         data = &data[pos + 1..];
        //     }
        // }

        entries
    }
}
