#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

mod aaa {
    // add code here
    const X: i32 = 10;
    fn print_aaa() {
        println!("{}", 42);
    }
}

mod ccc {
    pub fn print_ccc() {
        println!("{}", 25);
    }
}

fn  main() {
    use ccc::print_ccc;
    print_ccc();
}

