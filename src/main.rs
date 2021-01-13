use pancakedb::simple::Table;
// use pancakedb::mm::Table;

fn main() {
    let mut table = Table::new("awords.txt").unwrap();
    let res = table.find("lark".as_bytes());
    // for val in res {
    //     eprintln!("{:?}", std::str::from_utf8(val.data));
    // }
    eprintln!("{:?}", res.len());

    // table.insert(b"florp");
    // let res = table.find("florp".as_bytes());
    // eprintln!("{:?}", res.len());
}
