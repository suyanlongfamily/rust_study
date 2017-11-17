extern crate libc;

fn main() {
    println!("Hello, world!");
    unsafe { libc::srand(libc::time(0 as *mut i64) as u32) };
    println!("lib::rand = {}", unsafe { libc::rand() });
    println!("lib::pid = {}", unsafe { libc::getgid() });
    let var = 10;
    //    let var = &var as *const i32;//这种就可以强制转换了？？？？
    //    use_void(var as *const Void);
    let var = &var as *const i32;//这种就可以强制转换了？？？？
//    move { var };
    use_v(var as *const V);
    
    //以后让我们用指针编程吧。
    use_vec(var as *const Vec<i32>);
}

#[derive(Debug)]
pub enum Void {}

struct V {
    i: i32,
}

fn use_void(v: *const Void) {
    //这都可以，原来是这样使用的啊。
    unsafe { println!("{:?}", *(v as *const i32)) };
}

//不过细想来也是，只要是指针类型，否可以转换的，毕竟都是32\64位的指针。
fn use_v(v: *const V) {
    //这都可以，原来是这样使用的啊。
    unsafe { println!("{:?}", *(v as *const i32)) };
    println!("{:?}", v);
    unsafe { println!("{:?}", (v as *const i32)) };
}

// 如果是指针的话，其实就无关类型了，即，中间类型就无法紧要了。
// 转变成一个 (&K as *const K) as (*const T), => *(*const T as *const K),再转变过来，这个类型T是任何类型都可以，无关紧要，反正都是指针。
//只要不违反它的所有权。
fn use_vec(v: *const Vec<i32>) {
    //这都可以，原来是这样使用的啊。
    unsafe { println!("{:?}", *(v as *const i32)) };
    println!("{:?}", v);
    unsafe { println!("{:?}", (v as *const i32)) };
}