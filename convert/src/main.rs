
fn main() {
    use std::str::FromStr;
    let s = "125";
    let x = i32::from_str(s).unwrap();

    let sdf = format!("{}",123213);
    
    println!("---{:?}---",x);
    println!("Hello, world!");
}
