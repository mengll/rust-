### rust 引入命名空间mod 当前的开发目录下面 要有 libs.rs 或在libs 的目录下存在 mod.rs 的文件 
```
trait Good {
    fn show() ->i32;
    fn kill(&self,name:String);
}

struct Gm  {
    name:String,
    age:i32
}

impl Good for Gm {
    fn show()->i32{
        32
    }

    fn kill(&self,name:String){
        println!("{} killed {},age:{}",self.name,name,self.age); // 杀死了。。 年龄。。
    }
}

// 导入命名空间的数据
mod libs;
use libs::network;
// 使用了命名空间的操作
mod gg {
    pub mod kk {
       pub  fn kill(name:String){
            println!("太狠啦-{}",name);
        }
    }
}
// 调用测试 核心的使用
fn main(){
    let gm = Gm{name:String::from("mll"),age:31};
    gm.kill(String::from("pig"));
    let age = Gm::show();
    println!("the object age is {};",age);

    enum Coin {
        Penny,
        Nickel // 定义了未使用的变量
    }
    
    fn value_in_cnets(coin:Coin) ->i32 {
        match coin{
            Coin::Penny =>1,
            Coin::Nickel =>2
        }
    }

    let con = Coin::Penny;
    let vv = value_in_cnets(con);
    println!(" the match value {}",vv);
    gg::kk::kill(String::from("The big world")); // 包的方法的操作

    //  引入外部的包的
    network::connect(); // 调用模块命名
    network::disconnect();
    network::ip::getip();   //模块的引用 引入新的对象
}

--------libs mod.rs

pub mod network;

----libs/network/mod.rs
pub mod ip; //  使用第三方的=库的参数设置
pub fn ggip(){
    self::network::ip::getip();  // self 是相对路径 super 相对路径
}

pub fn connect(){
    println!("This  network {}","mll"); // 使用模块调用
}

// 切断当前的服务连接
pub fn disconnect(){
    println!("Disconnect is ok ");
}


---- libs/network/ip.rs
    pub fn getip(){
        println!("Thisisip")
    }

```
