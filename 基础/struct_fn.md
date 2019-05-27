### struct的方法的实现和相关的方法绑定，和相关的 trait的方法的绑定的实现
```
struct Dog {
    name :String,
    age :i8
}

impl Dog {
    fn run(&self){
        println!("{}",self.name);
    }
    
    fn eat(&self){
        println!("{} -- {}",self.name,self.age);
    }
}

// 实现一个对象
trait Hsq {
    fn get_name(&self)->String;
    fn set_name(&mut self,name:String);
}
impl Hsq for Dog{
    fn get_name(&self)->String{
        println!("dog--》{}",self.name);
        return self.name.clone(); // 当前的对象只能在内容只能在内部访问
    }

    fn set_name(&mut self,name: String){
        self.name = name;
    }
}

fn main(){
    let mut dog = Dog{name:String::from("dog"),age:3};
    dog.run();
    dog.eat();
    dog.set_name(String::from("lwd")); // 设置当前对象的名称
    let names = dog.get_name();
    println!("--->{}<----",names);
}

```
