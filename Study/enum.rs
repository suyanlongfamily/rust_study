 
#![allow(dead_code)]
 
enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    info { name: String, height: i32 },
}
 
//参数是一个Person枚举变量
fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is engineer"),
        Person::Scientist => println!("is scientist"),
        //内部结构 i 的值
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a Weight of {}", i),
 
        //name height参数，是内部结构它。 
        Person::info { name, height } => {
            println!("{} is {} tall", name, height);
            },
    }
}
 
fn main() {
    //看这个其实明白了，宏替换，变量的替换而已，
    let person: Person = Person::Height(18); //看到这个也就明白了，person枚举变量还是Person,只是取值只能取枚举类型给定的值。
    //Weight()元组的参数是一个整型100，可以看到编译器现实时无非就是，替换、宏展开而已，模式匹配而已，这些工作都是编译器完成的。
    //编译完成的，很重要！！！的理解！
    //看到这里，理解为什么说match 关键字的功能强大了。以及怎么使用枚举变量的，其实都是编译期间实现的。
    let amira = Person::Weight(100);
    let dave = Person::Scientist;
    let reb = Person::Engineer;
    //"Dave".to_owned() 这个函数的意思？？？
    let roban = Person::info {
        name: "Dave".to_owned(),
        height: 72,
    }; 
    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(reb);
    inspect(roban);
 
}
 