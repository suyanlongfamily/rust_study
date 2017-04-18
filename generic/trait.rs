
struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // fn create(name: &'static str);//存在静态函数时，就不能函数参数动态了。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{},is already naked...", self.name());
        } else {
            println!("{},is gets haricut!...", self.name());
            self.naked = true;
        }

    }
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }
}

impl Animal for Sheep {
    // fn create(name: &'static str){
    //     Sheep {
    //         name: name,
    //         naked: false,
    //     };
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah"
        } else {
            "baaaah!"
        }
    }
    //覆盖trait 继承过来的talk函数。
    fn talk(&self) {
        println!("{},pause briefly...{}", self.name, self.noise());
    }
}


struct Dog {
    naked: bool,
    name: &'static str,
}

impl Dog {
    fn new(name: &'static str) -> Dog {
        Dog {
            name: name,
            naked: false,
        }
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {

        // name = name + "dog";
        self.name
    }
    fn noise(&self) -> &'static str {

        self.name
    }
}


//trait参数可以实现多态性，
fn test_fun(arg: &Animal) {
    arg.talk();
    arg.noise();
    arg.name();
}

fn main() {
    //trict 接口 创建对象，而子类型做变量，有意思，有意思。C++面向对象的却不可以！！！不过也是类型推到原因，所以都是编译器自动实现了。
    //延伸一下，rust，怎么实习动态绑定的呢？？？
    // let mut dolly: Sheep = Animal::new("13123"); // dolly 必须显示标注类型。
    let mut dolly: Sheep = Sheep::new("suyanlong name"); // dolly 必须显示标注类型。
    let mut dog = Dog::new("i am a dog"); // dolly 必须显示标注类型。
    dolly.talk();
    dolly.shear();
    dolly.talk();
    {
        //trait存在静态函数时，就不能做函数参数引用，然后现实动态了。
        let dolly_trait: &mut Animal = &mut dolly;
        dolly_trait.talk();
    }
    //参数的动态分发，看到这里明白了。想要实现参数多态，trait不能有静态函数，必须全部实现里面的成员函数。
    //如果存在静态函数，&trait 对象不知道静态函数针对的是谁。所以静态函数一般可以放到impl里面，
    test_fun(&dolly);
    test_fun(&dog);

}