struct Good<'a> {
    name:&'a str
}

trait Fuck<'a> {
    fn get_good(&mut self);
    fn set_good(&mut self,nm:&'a str);
}
impl <'a> Fuck<'a>  for Good<'a> {
    fn get_good(&mut self){
        println!("{}",self.name)
    }

    fn set_good(&mut self,name:&'a str){
        self.name = name;
    }
}

fn main(){
    let mut a = Good{name:"youcano"};
    a.get_good();
    a.set_good("wenwen"); // 超出了当前苏游走的，作用的空间
    a.get_good();

}
