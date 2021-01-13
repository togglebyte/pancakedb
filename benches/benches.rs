#![feature(test)]
extern crate test;
use test::bench::{Bencher, black_box};
use pancakedb::simple::Table as SimpleTable;
use pancakedb::mm::Table as MmapTable;

#[bench]
fn create_simple(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn create_memmap(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn read_simple(b: &mut Bencher) {
    b.iter(|| {
        let mut table = SimpleTable::new("words.txt").unwrap();
        let records = table.find(b"lark");
        assert_eq!(records.len(), 61);
    });
}

#[bench]
fn read_memmap(b: &mut Bencher) {
    b.iter(|| {
        let mut table = MmapTable::new("awords.txt").unwrap();
        let records = table.find(b"lark");
        assert_eq!(records.len(), 61);
    });
}

#[bench]
fn update_simple(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn update_memmap(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn delete_simple(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn delete_memmap(b: &mut Bencher) {
    b.iter(|| {
    });
}
