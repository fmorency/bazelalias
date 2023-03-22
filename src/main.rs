fn main() {
    let a = merk_v1::Merk::open("/tmp/a.db").unwrap();
    let b = merk_v2::Merk::open("/tmp/b.db").unwrap();
}
