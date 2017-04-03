 #![allow(dead_code)]
 #![allow(unused_variables)]

struct S; //具体类型
struct GenericVal<T>(T); //泛型结构体。
// GenericVal 结构体实现。或者说扩充，
impl GenericVal<f32> {}
impl GenericVal<S> {}
impl<T> GenericVal<T> {}

struct Val {
    // val: mut f64,域不支持这种形式。
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

//扩充Val成员函数。
impl Val {
    // fn value(&mut self) -> &mut f64 {
    //     &mut((*self).val) //返回的是引用，只能读，没有所有权的转让。不可以返回一个可变引用吗？答：可以返回一个引用，要注意第一参数&self 改成 &mut self
    // }
    
    fn value(& self) -> & f64 {
        &((*self).val) //返回的是引用，只能读，没有所有权的转让。第一参数，我们就能够知道，我们只能读取，不能修该。
    }

    fn set_value(&mut self, val: f64) {
        (*self).val = val;
    }
}

// GenVal 针对泛型类型 `T` 的实现
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val //返回的是引用，只能读，没有所有权的转让。
    }
}

fn main() {
    let mut x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    {
        x.set_value(2.3333321); //设置属性。即在存在不可变借用时，原始值不允许修改。
    }
    // 不可以返回一个可变引用吗？
    let sadf = 1.332;
    // let mut x_values = x.value(); //这里左边的mut表明x_values 是一个可变的变量，
    // 不是说x.value()函数的返回值是可变的（mut），两个没有关系，这个也终于知道为什么要把类型放到变量的后面了。还是有原因的。    
    // x_values = &sadf;
    
    let x_values:& f64 = x.value();//显示的声明返回值类型。
    // let x_values = x.value();
    let y_values = y.value();
    // println!("values : x.value = {},y.value = {}",*x_values, *y_values);
    println!("values : x.value = {},y.value = {}",x_values, y_values);
}

// 突然想到为什么类型不放变量的左边，而放到右边。
// 细细一想：let [mut] XXX :[&mut T] = ***;
// 1、变量知识一个符号，代表内存的指代作用，其实都是地址，因此不需要真的变量有类型，只需要变量容纳的值只有可变与不可变两种，
// 2、变量指代的内存，却有类型，比如i32\i64\()\[]\Vec等等。因此把类型放到变量的右边，其实说的是等号右边值的类型（这个有分：可变与不可变、引用、类型）。
// 3、因此以上才是rust的重点吧。
