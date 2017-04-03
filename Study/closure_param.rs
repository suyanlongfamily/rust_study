fn apply<F>(f:F) where
    F:Fn(){
        f();
    }


fn main() {
    let x = 7;
    let priint = || println!("{}",x);
    apply(priint);    
}

