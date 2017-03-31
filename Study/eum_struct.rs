
//单元结构体
struct Nil;

//元组结构体
struct Pair(i32, f32);

//普通结构体
struct Piont {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Piont,
    p2: Piont,
}

fn main() {
    //实例化普通结构体对象。
    let piont: Piont = Piont { x: 0.3, y: 0.5 };
    //结构体，成员默认不是私有的吗？？？
    println!("piont info:({},{})", piont.x, piont.y);
    
    //使用let 解构。my_x ，my_y 是值，不是类型。
    let Piont { x: my_x, y: my_y } = piont;

    let _rectangle = Rectangle {
        //实例化，
        p1: Piont { x: my_x, y: my_y },
        p2: piont,
    };

    let _nil = Nil;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);



}

