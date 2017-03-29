// use vec

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 62, 12, 34, 6];
    for i in &v {
        println!("vector :{}", i);
    }

    {
        let mut foo = v.pop();
        // println!("pop :?", foo);
    }



    println!("-------------------");
    // for i in v {
    //     println!("vector :{}", i);
    // }
    ////为什么在第一次使用所用权遍历后，再使用所有权遍历就不能用了呢？？
    // println!("-------------------");
    // for i in v {
    //     println!("vector :{}", i);
    // }

    for i in &v {
        println!("vector :{}", i);
    }

    //这种不能通过编译了。
    // let v = vec![1, 2, 3, 4, 5];
    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }

    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }

        

}
