
//禁止无效代码生成。
#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

//默认继承关系。 
// #[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};
    use Work::*;
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money..."),

    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldier figth!"),
        // _ => println!("nothing"),//work 已经给定范围了，所以不需要再加defluat了。
    }

}


