use std::ops::{Deref};

struct Number<'a> {
    x:&'a i32
}

fn as_num_ref(x:&i32) ->Number<'_>{
    Number{x:&x}
}

// 数据类型的转化的操作
struct Gk<'a,T> {
    age:T,
    _name:&'a str
}

// 解析当前的对象可为空的操作 接口和命名空间的相关的操作的实现
    impl <'a,T> Deref for Gk<'a,T>{
        type Target=T;
        fn deref(&self) ->&T{
            return &self.age
        }
    }

    // 可变的解引用是不需要
//    impl <'a> DerefMut for Gk<'a> {
//        fn deref_mut(&mut self) ->&mut i32{
//            return &mut self.age
//        }
//    }

// 智能指针
struct Example {
    a:i32,
}

impl Drop for Example {
    fn drop(&mut self){
        println!("I example data:{}",self.a);
    }
}

impl Example {
    fn show(&self){
        println!("「世界那么大」{}",self.a)
    }
}

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    Nil
}

use List::{Cons,Nil};

// 高阶函数
fn divs<'b,T>(a:T,c:String) where for<'a> T:Fn(&'a str)->&'a str  {
    let s = a(c.as_str());
    println!("高阶函数===》{}",s);
}

fn makestr<'a>(name:&'a str) ->&'a str {
    name
}

fn as_str(data:&u32) ->&str{
    let s = format!("{}",data);
    &s
}

fn main(){
    println!("文件neirong");
    let xxx:i32 = 32;
    let x = as_num_ref(&xxx);
    println!("{}", x.x);
    let gk = Gk{_name:"frj",age:312};
    println!("{}",*gk);
    {
        let ex = Example{a:321};
        println!("{}",ex.a)
    }

    let bos = Box::new(Example{a:329});
    println!("{}",bos.a);
    bos.show();

    // 媒体的高阶函数的控制
    let list = Cons(1,Box::new(Cons(2,Box::new(Nil))));
    println!("{:?}",list);
    divs(&makestr,"fujing".to_string());
}

