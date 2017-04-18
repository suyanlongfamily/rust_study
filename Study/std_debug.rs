

// #[derive(Debug)]
struct Student {
    name: &'static str,
    old:i32,
    address:&'static str,
}

// 对Debug trait 进行实现。
use std::fmt::Debug;
impl Debug for Student {
    fn fmt(&self, &mut Formatter) -> Result{
        Ok(1)                              
    }
}

fn main() {
    let stu = Student{name:"suyanlong",old:27,address:"henan"};
    println!("{:#?}",stu);
    
}