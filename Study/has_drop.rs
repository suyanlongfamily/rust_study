
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("drop......");
    }
}
fn main() {
    let x = HasDrop;
}