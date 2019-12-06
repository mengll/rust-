struct Conf{
    address:String,
    num:i32
}

// 发生所有权的转移 在函数闭包的内部和参数对应的环境
fn with_opt(name:String) -> impl FnOnce(&mut Conf){
   let mc = move |conf:&mut Conf|{
       conf.address = name
    };
    mc
}

// 声明别名对象
type OP = fn(&mut Conf);

// 设置匹配模式
trait Option {
     fn set_address(&mut self,v:String);
}

// 奇葩的用法
fn duo_arg(op:OP) {
    let mut co = Conf{address:String::from("mmm"),num:1};
    op(&mut co);
    println!("test->{:?}",co.address);
}

impl Option for Conf {
     fn set_address(&mut self,v:String){
            self.address = v
    }
}

fn main(){
    let mut conf = Conf{address:"".to_string(),num :0};
    let fc= with_opt("shandong".to_string());
    fc(&mut conf);
    println!("1->{:?}",conf.address);
    let ff = conf.set_address("jack".to_string());
    println!("2->{:?}  - {:?}",conf.address);

    // 获取内容
    let cc= move | conf:&mut Conf|{
            conf.address = String::from("时间");
    };
    duo_arg(cc);
}
