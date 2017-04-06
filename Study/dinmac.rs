trait Person {
    fn new(init_i: i32) -> Self;
    fn display(&self);
    fn count(&mut self, i: i32) -> i32 {
        i
    }
    fn add_thing(&mut self, thing: i32) -> i32;
    fn push(&mut self, val: i32);
}


struct Name {
    name: String,
    old: i32,
    address: String,
}

impl Person for Name {
    fn new(init_i: i32) -> Name {
        Name {
            name: "suyanlong".to_string(),
            old: init_i,
            address: "河南".to_string(),
        }
    }

    fn display(&self) {
        println!("name = {},old = {},address = {}",
                 (*self).name,
                 (*self).old,
                 (*self).address);
    }

    fn add_thing(&mut self, thing: i32) -> i32 {
        (*self).old = thing;
        thing
    }

    fn push(&mut self, val: i32) {
        // *self.old = val;// 这个 . (点成员符号)优先级要高于*
        (*self).old = val;
    }
}

fn main() {
    println!("-------begin------");
    // let val:Name = Person::new(12);//可以通过这种方式，但是绑定必须指定类型，编译器才能自动推到
    let mut val = Name::new(21); // 申请了一个默认不可变的绑定
    val.display(); // 可变绑定可以调用不可变的成员，但是，不可以绑定不能调用产生可变的成员。

    // trait在编译阶段没有固定大小，我们不能直接使用trait当作实例变量、参数、返回值。
    // 这个也就表明了不能类是java、C++一样实现多态，基类引用做参数，运行时进行多态的绑定。
    // let obj:Person = val;

    val.add_thing(11);
    val.display();

    println!("-------end------");


}