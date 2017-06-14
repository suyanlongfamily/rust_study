
use std::ops::Add;
#[derive(Debug)]
pub struct Piont {
    //不在同一个模块mod内的,不能访问字段,即使在同一个crate,也不行,这就是为什么要提供一个new函数了.
    //mod << crate
    x: i32,
    y: i32,
}

impl Piont {
    pub fn new() -> Piont {
        Piont { x: 0, y: 0 }
    }
}

impl Add for Piont {
    type Output = Piont;
    fn add(self, p: Piont) -> Piont {
        Piont {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}
