use std::env;

fn main() {
    println!("Hello, world!");
    let argumt:Vec<String> = env::args().collect();//迭代器的使用，看看那要好好学习一下了。
    println!("---{:?}---",argumt);
    
    


}
