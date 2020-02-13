trait GetName{
    fn get_name(&self)->String;
}
trait PrintName{
    fn print_name(&self);
}

impl <T:GetName>  PrintName for T {
    fn print_name(&self){
        println!("{}","dingyie")
    }
}

struct Student {
    name:String,
}

impl GetName for Student {
    fn get_name(&self)->String{
        println!("{}","getName the trait 的接口的形式");
        "one".to_string()
    }
}

// 有条件的实现相关的特征的实现的方式和方法的过程
fn main(){
    println!("{}","the best");
    let s = Student{name:String::from("one best")};
    s.get_name();
    s.print_name();
}
