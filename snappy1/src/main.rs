
mod snappy;
use snappy::*;
fn main() {
    let data = vec![1,2,3,4];
    let undata = compress(&data);
    println!("Hello, world! {:?}--",data);
    valid();
    empty_compress();
    
}
fn valid() {
    let d = vec![0xde, 0xad, 0xd0, 0x0d];
    let cd = compress(&d);
    assert!(validate_compressed_buffer(&cd));
    let dd = decompress(&cd).unwrap();
    assert!(&d == &dd);
}


fn empty_compress() {
    let d = vec![];
    let cd = compress(&d);
    assert!(validate_compressed_buffer(&cd));
    let dd = decompress(&cd).unwrap();
    assert!(&d == &dd);
}