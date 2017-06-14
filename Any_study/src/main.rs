
#![feature(get_type_id)] //这个必须放到前面不然也会报错的。
#![feature(slice_concat_ext)]
use std::any;
use std::any::Any;
use std::any::TypeId;
//一个类，类型的类。类似java的class类一样，不过这个是有限制的。
// 看下面的规则，对所有的类型：
// impl<T> Any for T
// where T: 'static + ?Sized
mod piont;
use piont::Piont;
use std::convert::Into;

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
    let tule: Result<(i32, bool), ()> = std::result::Result::Ok((12, true));
    let cal = 23;
    if let Ok((cal, true)) = tule {
        println!("{:?}", cal);
    } else {
        //这种结构,不行的,上面的cal不能用.,
        //上面结构出来的只能是一个局部变量.
        println!("{:?}", cal);
    }


    //-------------------------------------------------
    //这个是转移所有权,并且实现了copy,可以推断,闭包实现了,FnOnce trait,即self,消耗!!!
    let x = 123;
    let cloure = |mut y: i32| {
        y = x + 12;
        y
    };
    println!("--{:?}--", cloure(2));

    println!("{:?}", x);

    //-------------------------------------------------
    let mut x = 123;
    {
        //这里对引用的上下文进行修改,借用绑定,所以并且对原值进行了修改,所以是FnMut trait,即&mut self.
        let mut cloure = |y: i32| {
            x = x + 12;
            y
        };
        println!("--{:?}--", cloure(2));
    }

    println!("{:?}", x);

    //--------------------------------------------------
    let piont = Piont::new();
    println!("---piont = {:?}", piont);
    let piont1 = Piont::new();
    {
        let cl_piont = || piont; //可以看到,是实现了移动,因为上下文做了返回值,转移了所有权.把上下文的捕获的内容,消耗了.
        cl_piont();
    }
    // println!("---{:?}---", piont);

    //-----------------------------------------------------
    let mut piont = Piont::new();
    println!("---piont = {:?}", piont);
    let piont1 = Piont::new();
    {
        let cl_piont = || {
            let po = &piont; //仅仅是引用.所以时Fn类型.
            //piont;
        };

        cl_piont();
    }
    println!("---{:?}---", piont);

    //看来,闭包会根据闭包的内容,进行实现是 FN\FnMut\FnOnce,三种类型.
    //1 如果对上下文进行借用,闭包就是Fn类型
    //2 如果对上文进行可变的借用,闭包就是FnMut类型
    //3 如果对上文进行所有权的转移就是FnOnce类型
    use std::boxed::Box;

    let err = ErrorCode::AuthErr;
    println!("---   {:?}---", err.int_to_string());


    let err = ErrorCode::AuthErr;
    let strq: String = err.into();
    println!("{:?}", strq);



}

//枚举类型两层含义,一个是字符串,一个时代表的值.

#[derive(Debug, Clone)]
pub enum ErrorCode {
    Success = 00000,
    TimeOut = 99999,
    InvalidReq = -32600,
    MethodErr = -32601,
    DoubleTx = -32602,
    DbError = -32603,
    AuthErr = -32604,
    InvalidParams = -32605,
}

impl ErrorCode {
    pub fn int_to_string(self) -> String {
        let err_int: i64 = self as i64;
        err_int.to_string()
    }
}

impl Into<String> for ErrorCode {
    fn into(self) -> String {
        format!("{:?}", self)
    }
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
