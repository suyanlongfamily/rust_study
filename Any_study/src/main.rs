
#![feature(get_type_id)] //这个必须放到前面不然也会报错的。
#![feature(slice_concat_ext)]
use std::any;
use std::any::Any;
use std::any::TypeId;
//一个类，类型的类。类似java的class类一样，不过这个是有限制的。
// 看下面的规则，对所有的类型：
// impl<T> Any for T
// where T: 'static + ?Sized


fn main() {

    println!("Hello, world!");
    is_str(&"std_unicode".to_string());

    let str_any = String::from("sdf");
    // is_type_id(&0);
    // println!("----String_type_id--{:?}", is_type_id(str_any.as_ref()));

    //注意类型写法。
    println!("----String_type_id--{:?}", TypeId::of::<String>());

    use std::slice::Split;
    use std::slice::SliceConcatExt;
    //使用场景：可以用在读取指定文件的内容。
    let file_content = include_bytes!("./main.rs"); //&'static [u8; N]
    println!("file_content = {:?}",
             String::from_utf8(file_content.to_vec()));

    get_code_detile();


}


fn get_code_detile() {
    // let this_file = file!();
    // println!("defined in file: {}", this_file);

    let line = line!();
    let file_name = module_path!();
    let column = column!();
    println!("---{:?}--{:?}---{:?}--", file_name, line, column);
}

fn is_type_id(s: &Any) -> bool {

    println!("---{:?}---", s.get_type_id());
    TypeId::of::<String>() == s.get_type_id()


}

fn is_str<T: any::Any>(param: &T) {
    let param = param as &any::Any; //必须强制显示的转换。
    match param.downcast_ref::<String>() {
        Some(as_string) => {
            println!("---{:?}---", as_string);
        }
        None => {
            println!("---none--");
        }
    }

}




// #![feature(get_type_id)]

// use std::any::{Any, TypeId};

// fn is_string(s: &Any) -> bool {
//     TypeId::of::<String>() == s.get_type_id()
// }

// fn main() {
//     assert_eq!(is_string(&0), false);
//     assert_eq!(is_string(&"cookie monster".to_string()), true);
// }
