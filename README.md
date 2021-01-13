# The plan

TODO: rename entry to record

Add benchmarks

## Overview

Each table is a file. 
Each row in a file is an entry.

## Misc notes

* To delete a record we write over the old data with a tombstone.
* To update a record we insert new data **as** the old record, and delete 
  the old record.
* To clean up tombstones we make a sweep every now and then
* Define a tombstone

## Unknowns

* Q: Do we need to lock the file before writing?
* Q: What happens when we write to a "memmap" file that has changed externally
* Q: How do we delete a record without removing data: 
  A: A toombstone according to ACouncilOfOne

## Structs

```rust
struct Table {
    inner: File,
}

impl Table {
    fn find_row(&self, key: &[u8]) -> &[u8] {
    }
    
    fn delete(&mut self, index: ?) {
        // replace data in buffer with tombstone data
    }
    
    fn update(&mut self, data: &[u8], old_index: ?) {
        self.delete(old_index);
        self.insert(data);
    }
    
    fn insert(&mut self, data: &[u8]) {
        // Will always append
    }
}

struct Tombstone;

impl Tombstone {
    fn new(size: usize) -> Vec<u8> {
        vec![0; size]
    }
}
```

## Create

### Rust simple version:
1. Append data

### Magic version:
1. Append data 

## Read: 

Look something up:
1. Memmap the file
2. Find first character in keyword using `netlib::memchr::memchr`
3. Split the buffer on the position
4. try to match the first set of bytes in the new chunk, with keyword[1..]
5. Find the `\n` to the left and right of the first byte position

## Update:

### Rust simple version:
1. Find the row
2. Replace it with the updated row

### Eccentric version:
1. Find the row
2. Replace the row with a tombstone
3. Append new data


## Delete:

### Rust simple version:
1. Locate data to delete
2. Replace data with tombstone

### Florped version:
1. Locate data to delete
2. Replace data with tombstone
