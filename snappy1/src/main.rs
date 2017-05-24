#![feature(libc)]
mod snapp1;
use snapp1::*;
fn main() {
    let data = compress(&vec![1,2,3,4]);
    println!("Hello, world! {:?}--",data);
}
