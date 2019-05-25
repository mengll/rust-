###  struct 的组合的方式执行参数
```
// 创建结构体的嵌套
fn main(){
    struct Inner {
        name:String,
    }

    impl Inner {
        fn get_name(&self)->String{
            return String::from("inneir data");
        }
    }
    
    struct Gk {
        name:Inner,
        skill:String
    }

    impl Gk {
        fn skill(&self,skill:String){
            println!("name: {} -- skill: {}",self.name.name,skill);
        }
    }

    let geek = Gk{name:Inner{name:String::from("mll")},skill:String::from("go python")};
    let user_name = geek.name.get_name();
    println!("{}",user_name);
    geek.skill(String::from("go python js lua js ......"));
}

```
