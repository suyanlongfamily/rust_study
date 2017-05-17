
pub trait CallBackFun {
    fn call_back(&self);
}

// impl CallBackFun for T {//这种无法实现编译，不符合孤儿原则。
//     fn call_back(&self){
//         println!("-----{:?}----",self);
//     }
// }

//原来这样也可以的，注意impl 泛型的实现，一定要要添加泛型<X>参数。!!!!!
// 1、当前的crate内，注意。孤儿原则的规则。
// 2、T 代表整个crate 的类型，其实都可以调用，但是必须满足约束条件，即T类型必须实现某个或者多个trait。
// 3、思考到这里，如果要想某个类型能调用指定的trait里面的方法，我们可以给他满足制定的约束就可以了。又一种角度思考。
// 4、如果要想某一个类型，要具有某个trait的方法,我们就实现指定的trait.
impl<T> CallBackFun for T {
    fn call_back(&self) {
        let var = self;
        let vat2 = self;
        self;
    }
}


// impl CallBackFun for i32 {
//     fn call_back(&self){
//         println!("-----{:?}----",self);
//     }
// }

// use std::ops::Deref;

