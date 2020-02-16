use std::ops::Deref;
struct MyBox<T>(T); // 泛型结构体的实现的元组结构体 解引用的实现的

impl <T> MyBox<T>{
    fn new(x:T)->MyBox<T> {
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) ->&T { // 不获取当前对象的借用的的方式
        &self.0
    }
}

/// 泛型结构的解构函数的实现
fn main(){
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
    println!("泛型结构体的实现")
}
