
fn main() {
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];
    // println!("2,in vec1:{}", vec1.iter().any(|&x| x == 2));

    // println!("2,in vec2:{}", vec2.into_iter().any(|x| x == 2));
    // let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];
    // println!("2,in array1:{}", array1.iter().any(|&x| x == 2));
    // println!("2,in array2:{}", array2.into_iter().any(|&x| x == 2));
// Vec
    let v1 = vec![1, 2, 3];

    // let v2: Vec<i32> = 
//     v1.iter().map(|x| {
//         println!("---{}",x);//验证遍历
//         x + 1
//     }
//   );

    // assert_eq!(v2, [2, 3, 4]);
    // println!("v1 = {:?}", v1);
    // println!("v2 = {:?}", v2);

// 当讲到Iterator的定义时，我们故意省略一个小的细节。Iterator定义了一系列默认实现，
// 他们会调用next方法。因为next是唯一一个Iterator trait 没有默认实现的方法，
// 一旦实现之后，Iterator的所有其他的适配器就都可用了。
// 这些适配器可不少！

let a = [1, 2, 3];
assert_eq!(a.iter().count(), 3);

let a = [1, 2, 3, 4, 5];
assert_eq!(a.iter().count(), 5);

}