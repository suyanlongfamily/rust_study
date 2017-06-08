
mod snappy;
use snappy::*;
fn main() {
    let data = vec![1,2,3,4];
    let undata = compress(&data);
    println!("Hello, world! {:?}--",data);
    valid();
    empty_compress();

    if cfg!(debug){
        println!("----debug---");
    }else {
        println!("----relase---");
    }
    // fun();


    if cfg!(debug_assertions) {
        println!("slow2")
    } else {
        println!("fast2")
        }


        // do_something();
}


#[cfg(debug)]
fn fun(){
    println!("-----debug");
}

#[cfg(build = "release")]
fn fun(){
    println!("-----release");
}

fn valid() {
    let d = vec![0xde, 0xad, 0xd0, 0x0d];
    let cd = compress(&d);
    assert!(validate_compressed_buffer(&cd));
    let dd = decompress(&cd).unwrap();
    assert!(&d == &dd);
}


#[cfg(build = "debug")]
fn do_something1(){
// Output sensitive debugging info
  println!("-----debug");
}

#[cfg(build = "release")]
fn do_something(){
// Don't output anything
  println!("-----release");
}


fn empty_compress() {
    let d = vec![];
    let cd = compress(&d);
    assert!(validate_compressed_buffer(&cd));
    let dd = decompress(&cd).unwrap();
    assert!(&d == &dd);
}