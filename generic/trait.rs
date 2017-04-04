
struct Sheep {
    naked:bool,
    name:&'static str,
}

trait Animal{
    fn new(name:&'static str)->Self;
    fn name(&self)->&'static str;
    fn noise(&self)->&'static str;
    
    fn talk(&self){
        println!("{} says {}",self.name(),self.noise());
    }
}

impl Sheep {
     fn is_naked(&self)->bool{
         self.naked
     }
     
     fn shear(&mut self){
         if self.is_naked() {
             println!("{},is already naked...",self.name());
         }else {
             println!("{},is gets haricut!...",self.name());
             self.naked = true;
         }
         
     } 
}

impl Animal for Sheep {
    
    fn new(name:&'static str) -> Sheep{
        Sheep{name:name,naked:false}
    }
    
    fn name(&self) -> &'static str{
        self.name
    } 
    
    fn noise(&self) -> &'static str{
        if  self.is_naked() {
            "baaaaah"
        }else{
            "baaaah!"
        }
    }
    
    fn talk(&self){
        println!("{},pause briefly...{}",self.name,self.noise());        
    }
        
}

fn main() {
    //trict 接口 创建对象，而子类型做变量，有意思，有意思。C++面向对象的却不可以！！！
    //延伸一下，rust，怎么实习动态绑定的呢？？？
    let mut dolly:Sheep = Animal::new("13123");// dolly 必须显示标注类型。
    dolly.talk();
    dolly.shear();
    dolly.talk();
}