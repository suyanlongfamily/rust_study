
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let val = 100;
    let kk = 10000;
    println!("----{:?}---", 100);

    let mut count = 10000;
    while count > 0 {
        count = count - 1;
        println!("---{:?}---", count);
        let dur_time = Duration::new(6, 0);
        thread::sleep(dur_time);

    }

}

#[cfg(test)]
mod test {
    #[test]
    fn test_fun(){
        let kk = 1000;
        let val = 123123;
        let sadfsadf = 10000;
        println!("----{:?}----",sadfsadf);

    }
}

