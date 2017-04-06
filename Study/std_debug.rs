

#[derive(Debug)]
// std::format::Debug
struct Student {
    name: &'static str,
    old:i32,
    address:&'static str,
}

// use std::fmt::Debug;

// impl Debug for Student {
//     fn fmt(&self, &mut Formatter) -> Result{
                                    
//     }
// }



fn main() {
    let stu = Student{name:"suyanlong",old:27,address:"henan"};
    println!("{:#?}",stu);
    
}