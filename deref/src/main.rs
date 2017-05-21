
// use std::rc::*;

// fn main() {
//     string_bar();
//     string_deref();
//     println!("Hello, world!");
//     let varcon = 2;
//     println!("----{}---", varcon);
//     let ni = 2;
//     // 诺斯费都杀掉

//     let s = "hello";
//     println!("length: {}", s.len());
//     println!("length: {}", (&s).len());
//     println!("length: {}", (&&&&&&&&&&&&&s).len());

//     println!("----length: {}", str::len(&s)); //注意这种形式可以的。

//     // 自动deref的规则是，如果类型T可以解引用为U，即T: Deref<U>，则&T可以转为&U。
//     let s = Rc::new(String::from("hello"));

//     println!("length: {}", s.len());
//     // println!("length: {}", s.deref().len());
//     // println!("length: {}", s.deref().deref().len());

//     println!("length: {}", (*s).len());
//     println!("length: {}", (&*s).len());
//     println!("length: {}", (&**s).len());


// }

// use std::string::String;
// #[warn(non_snake_case)]
// use std::ops::Deref;
// fn string_deref() {
//     let str_tmp = String::from("123123");
//     let str_tmp_deref: &String = &str_tmp; //指针类型
//     // let str11 = *&str_tmp; //不能这样做，会被转移所有权的。看来是对引用的的deref()返回值进行解引用。

//     let str11 = &*&str_tmp; //引用
//     let str112 = str_tmp.deref(); //可以用。类型是&str类型了。
//     let str111 = (&str_tmp).deref(); //deref() 返回一个引用。类型是&str类型了。
//     let str222 = (&&str_tmp).len(); //deref() 返回一个引用。

//     // let str222 = (&&str_tmp).deref(); //deref() 返回一个引用。
//     let str3 = &&str_tmp; //指针的指针，二维指针。
//     let str33 = *str3; //
//     let str331 = str33.deref();
//     let str333 = str3.deref(); //
//     let str4: &str = *str3;

//     // println!("---类型 = {:?}", std::intrinsics::type_name());
//     // core::intrinsics::type_name
//     // let str_12 = *str_tmp;//无法编译。解引用之只能对引用类型起作用。所以知乎的一篇文章时有错误的。即：
//     // str  => String
//     // *str  => 错误。
//     // &str => &String
//     // &*str =>
//     // println!("-----{:?}----{:?}---{:?}---{:?}---",
//     //          *&str_tmp,
//     //          str_tmp.deref(),
//     //          (&&&&str_tmp).deref(),
//     //          (str_tmp));

//     let kkk = 12;
//     let kkk2 = 12;
// }


// fn string_bar() {
//     let str_tmp = String::from("1231231222222222222");
//     let k2 = &str_tmp;
//     let k22: &str = &*&&&&(k2);
//     // str_tmp.deref();
//     // let kk = &*str_tmp;
//     // let kkkk = &*&str_tmp;
//     let kk = str_tmp.deref();


//     let int_val: &mut i32 = &mut 23;
//     *int_val = 21;

//     // int_val.deref();
//     let wq = 1;

// }




// use std::any::{Any, TypeId};
// fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
//     TypeId::of::<String>() == TypeId::of::<T>()
// }





// use std::ops::Deref;

// struct Mp3 {
//     audio: Vec<u8>,
//     artist: Option<String>,
// title: Option<String>,
// }

// impl Deref for Mp3 {
//     type Target = Vec<u8>;

//     fn deref(&self) -> &Vec<u8> {
//         &self.audio
//     }
// }

// fn main() {
//     let my_favorite_song = Mp3 {
//         // we would read the actual audio data from an mp3 file
//         audio: vec![1, 2, 3],
//         artist: Some(String::from("Nirvana")),
//         title: Some(String::from("Smells Like Teen Spirit")),
//     };
//     // 一个实例却可以解引用。。。。
//     //这样说明了什么？编译器，进行扩展，转换成这样形式 *(my_favorite_song.deref())
//     assert_eq!(vec![1, 2, 3], *my_favorite_song);
// }

use std::ops::Deref;
use std::fmt::Debug;

#[derive(Debug)]
struct DerefExample<T>
    where T: Debug
{
    value: T,
}

impl<T> DerefExample<T>
    where T: Debug
{
    fn display(&self) {
        //看看这两个调用意思，是什么意思了。
        println!("----dispaly---{:?}---", *self); //这一次是对self本身是引用类型，先对它解引用
        println!("----dispaly---{:?}---", **self); //第一次时变成所有者，第二次是对所有者进行转换，调用自定义的解引用。显示的调用*操作符。
    }
}

impl<T> Deref for DerefExample<T>
    where T: Debug
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

fn foo(arg: &char) {
    println!("----foo---{:?}", arg);
}

fn main() {
    let x = DerefExample { value: 'a' };
    // let y = *x; //这个是什么意思呢？显示的调用？？？
    assert_eq!('a', *x);
    // println!("-y = {:?}----", &x);//宏时展开的。
    println!("-y = {:?}----", *x); //显示的解引用调用。
    x.display();

    println!("-----------");
    let chark = *x; //进行copy了，
    println!("---*x = {:?}---", *x); //这里对一个所有者解引用，编译器发现存在自定义的deref，会进行先调用deref()，所以是char类型，

    println!("---&x = {:?}---", &x); //这里不必多说了。DerefExample类型
    foo(&x); //这个时编译器的自动转换，由＆DerefExample类型转换成　&char类型。

    println!("---*&x = {:?}---", *&x); //这里是DerefExample类型 为什么呢？因为 *& 相互抵消了。
    println!("---&*x = {:?}---", &*x); //这里是&char类型,方向自右向左运行，先自调用自定义的deref函数。
    foo(&*x); //*x 这个返回的值时 char类型，所以需要改成引用，即&char。所以还不如上面的过瘾。明确。
    foo(&*&x); //与上面的同样的道理，可为什么多了一个&,先相互抵消，在隐式的转换。

    // 总结：1、自定义解引用主要是对引用类型进行隐式的自动转换：如果 U impl Deref<Target = T> 则 &U -> &T 。这个才是重点。
    //      2、对 * 操作符，在拥有者显示的调用时，相当于调用自定义的方法，即对一个*x，先做一次转换变成：*(x.deref())；
    //         所以变成了 T类型，而不是引用类型。
    //      3、对一个引用类型，对它解引用就会调用默认的
    //          impl<'a, T: ?Sized> Deref for &'a T {
    //                  type Target = T;
    //                  fn deref(&self) -> &T { *self }
    //          }

    let str_ref: &char = &x; //这个厉害了，这个才是Deref的真正用法，其他的介绍用法都是扯淡啊。


}

