use std::ops::Deref;


trait Animal {
    fn eat(&self);
    fn run(&self);
}

struct Dog<'a> {
    name:&'a str,
    age:i8
}

impl <'a> Animal for Dog<'a> {
    fn eat(&self){
        println!("The Dog is eating :{}",self.name)
    }
    fn run(&self){
        println!("The Dog is run :{} and age is {}",self.name,self.age)
    }
}

struct Cat<'a>{
    name:&'a str,
    age:i8
}

impl <'a> Animal for Cat<'a>{
    fn eat(&self){
        println!("The Cat is eating :{}",self.name)
    }
    fn run(&self){
        println!("The Cat is run :{} and age is {}",self.name,self.age)
    }
}

#[derive(Debug)]
struct conf{
    add:String,
    port:i32,// 设置端口
}

type Opt = dyn for <'a> Fn(&'a mut conf);

fn set_addr()-> Box<Opt> {
     Box::new(|cf:&mut  conf|{
         cf.add = "daa".to_string()
     })
}

fn set_port(port:i32) ->Box<Opt> {
    Box::new(move |cf:&mut conf|{
        cf.port= port
    })
}

// 不确定的大小类型
fn setconfig(op:Vec<Box<Opt>>){
    let mut c =conf{add:"".to_string(),port:0};
    for k in op.iter() {
        k(&mut c)
    }
    println!("ender:{:?}",c);
}

fn main(){
    let dog = Dog{name:"小强",age:3};
    dog.run();
    let op:Vec<Box<Opt>> = vec![set_port(32),set_addr()];
    setconfig(op)
}

//
