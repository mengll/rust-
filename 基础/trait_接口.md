### rust中的声明 golang中的接口
```// trait 特征 特点 特性
// 类比此声明为golang中的interface 
// 在Rust 所有的trait中都有一个隐藏的类型Self(大写的S)代表当前这个实现了些trait 的具体类型，trait中
// 定义的函数也可以称作关联函数，函数的第一个参数如果是Self相关的类型，且命名为self(小写)这个参数可以被称为
// 接收者 具有接收者的参数的函数，称为方法可以通过变量实例，使用小数点的来调用 
// 没有接收者的函数 被称为静态函数可通过类型加上双引号的方式调用
// Rust 中Self (大写的S) 和self(小写的s)都是关键字，Self 表示类型 self 表示变量名
trait T {
    fn method_one(self:&Self);
    fn method_two(self:&Self);
    fn method_three(&self);
    fn method_four();
}

// 声明一个结构
struct Gk {
    name:String,
    age :i8
}

// 相当于golang 接口与 结构体的绑定
impl T for Gk {
    fn method_one(self: &Self){
        println!("{}-age= {}",self.name,self.age);
    }

    fn method_two(self: &Self){
        println!("{}--age ={}",self.name,self.age);
    }

    fn method_three(&self){
        println!("{}----3--age ={}",self.name,self.age);
    }

    fn method_four(){
        println!("{}","method静态方法");
    }
}

fn main(){
    let geek  = Gk{name:String::from("mengll"),age:18};
    geek.method_one();
    geek.method_two();
    geek.method_three();
    Gk::method_four();
}
```

### 结构函数方法的接口的调用
```
#[derive(Debug)]
struct Foo;

trait Bar{
  fn baz(&self);
}

impl Bar for Foo {
  fn baz(&self){
    println!("{:?}",self)
  }
}

// 泛型，指定实现了相关的接口
fn static_dispatch<T>(t:&T) where T:Bar{
  t.baz();
}

// 类比golang 的interface{}接口的处理
fn dynamic_dispatch(t:&Bar){
  t.baz();
}

fn main(){
  let foo = Foo;
  static_dispatch(&foo);
  dynamic_dispatch(&foo); // 传递当前的对象实现相关的接口
}

```



