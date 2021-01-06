struct Employee {
    name :String,
    company:String,
    age:u32
}

impl Employee {
    fn show(&self){
        println!("{}",self.company)
    }
    fn kill(&mut self,name:String){
        println!("{}",self.name);
        self.name = name;
        println!("{}",self.name)
    }
}

// 创建接口计算数据
trait Gk {
    fn jj(&self);
    fn jk(&self){
        println!("{}","接口方法内部的调用")
    }
}

// 执行结构体的操作
impl Gk for Employee {
    // 数据当前对象的年龄
    fn jj(&self){
        println!("{}",self.age)
    }
}

// 函数返回结构体
fn new_struct() ->Employee{
    Employee{
        name:String::from("name"),
        company:String::from("two"),
        age:12
    }
}

fn main(){
    let instance_name = Employee{name:String::from("name"),company:String::from("asd"),age:1};
    println!("{}",instance_name.age);
    instance_name.show();
    instance_name.jj();
    instance_name.jk();
    
    let change_name:&mut Employee = &mut Employee{..instance_name};
    change_name.kill("fu".to_string());
    let cc =  &mut new_struct();
    cc.kill("fj".to_string());
}


// 数据关联类型
trait Run {
    type Gs;
    fn gorun(&self,_:Self::Gs);
    fn runrun(&self){
        println!("I can run ")
    }
}


struct G<'a> {
    name:&'a str
}

impl <'a> Run for G<'a> {
    type Gs = Gk<'a>; 
    fn gorun(&self,a:Self::Gs){
        println!("{:?}",a)
    }

    fn runrun(&self){
        println!("riernas");
    }
}

// 调用
    let gg = G{name:"dand"};
    let gk = Gk{muid:"fk",name:"fucker".to_string()};
    gg.gorun(gk);
    gg.runrun();




