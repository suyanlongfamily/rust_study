//match 关键字的学习。
fn main() {
    let foo = 3;
    match foo {
        1 => print!("----first---{}",foo),
        2 => print!("----second---{}",foo),
        3 => print!("----three---{}",foo),
        4 => print!("-----four--{}",foo),
        _ => print!("-------------end!"),
    }
}