mod aaa;//相当于在aaa.rs 文件中使用mod aaa{...} 一样
use aaa::print_aaa;
fn main() {
    println!("Hello, world!");
    print_aaa();
}
