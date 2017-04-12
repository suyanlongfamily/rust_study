use std::cell::Cell;

fn main() {
    let c = Cell::new(5);
    let five = c.get();

}