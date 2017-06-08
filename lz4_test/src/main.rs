
extern crate lz4;
use std::iter::FromIterator;
use std::env;
use std::fs::File;
use std::io::Result;
use std::io::Read;
use std::io::Write;
use std::path::Path;


use std::io::{Cursor, Error, ErrorKind};
use lz4::{Encoder, EncoderBuilder};
use lz4::Decoder;

fn main() {
    println!("LZ4 version: {}", lz4::version());
    test_encoder_smoke();




}

fn test_encoder_smoke() {
    let mut encoder = EncoderBuilder::new().level(1).build(Vec::new()).unwrap();
    let mut expected = Vec::new();
    expected.write(b"Some data").unwrap();
    println!("---{:?}----", String::from_utf8(expected.clone()));

    println!("---{:?}----", expected);
    encoder.write(&expected).unwrap();
    let (buffer, result) = encoder.finish();
    println!("---{:?}----", buffer);


    let mut decoder = Decoder::new(Cursor::new(buffer)).unwrap();
    let mut actual = Vec::new();
    decoder.read_to_end(&mut actual).unwrap();
    println!("---{:?}----", actual);
    println!("---{:?}----", String::from_utf8(actual.clone()));
    assert_eq!(expected, actual);

}
