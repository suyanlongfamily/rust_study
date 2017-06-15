

use std::fmt::Write;
fn main() {
    println!("Hello, world!");
    let mut name = String::from("");
    let result = write!(name, "123");
    writeln!(name, "123123123213123");
    writeln!(name, "123123123213123");
    println!("-----name = {:?}--", name);
    println!("---{:?}---", result);



}
