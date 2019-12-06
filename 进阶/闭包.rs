struct Conf{
    address:String,
    num:i32
}

// 发生所有权的转移
fn with_opt(name:String) -> impl FnOnce(&mut Conf){
   let mc = move |conf:&mut Conf|{
       conf.address = name
    };
    mc
}

// 设置匹配模式
trait Option {
     fn set_address(&mut self,v:String);
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
    println!("2->{:?}",conf.address)
}
