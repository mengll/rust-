### rust 中match的匹配是很强大的
```
use std::fs::File;
use std::io::ErrorKind;
// match 极为强大的控制流控制运算符，允许我们将一个值与一系列的模式相比较，并根据匹配模式，执行想对应的代码，匹配额模式
// 可以使字面量 ，变量 通配符，其它数据内容结构
enum Coin {
    Penny,
    Nickel
}

fn value_in_cnents(coin:Coin)->u32{
    match coin {
        Coin::Penny => 1, // 后面是一个表达式的值，或者
        Coin::Nickel =>2
    }
}

// 函数的类型，返回值的类型
fn opt(val:Option<fn()>)->Option<i32>{
    match val {
        Some(i) =>{i();
            Some(6)
        },
        None => None,
    }
}

// 模式匹配
fn main(){
    let f = File::open("src/hello.txt");
    let f = match f {
        Ok(file) => file, // 输出文件额内容的操作
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) =>fc,
                Err(err)=>panic!("There was a problem openning the file ")
            },
            other_error => panic!("") // 处理集中常见的错误
        }
    }; 
    println!("---->{:?}",f); 

    let five = Some(5);
    fn fk(){
        println!("optione fn type")
    }
    let six  = opt(Some(fk)); // Some 的值是任意类型的 可以带有参数的函数
    let none  = opt(None);
    println!("six:{}",six.unwrap())
}

// 高性能的
// rust 要求match需要对所有情况做完整的，无遗漏的匹配，如果遗漏编译无法通过，_ 可以使用占位符 模式匹配 和值匹配 
match 的语法结构也同样可以是表达式的一部分   match的每一个分支可以是表达式 要么用大括号括起来，要么使用逗号分开，每个分支都有相同的类型 match的后面没有分号，表达式的值将会，作为整个函数的返回值传递出去

fn main(){
    let x= 5_i32;
    match x{
        ref r => println!("Got a refrence to {}",r),
    }

    let y = 23;
    let c = match y{ 
        y => y+1
    };
    println!("c={}",c);

}
```
