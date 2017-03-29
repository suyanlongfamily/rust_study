fn main() {
    func2();
}

fn func1() -> Vec<isize> {
    let v = vec![1, 2, 3];
    v
}
fn func2() {
    let v = func1();
}
