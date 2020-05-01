struct Person {
    name:String,
    age:u8
}

fn main(){
    let p = Person{name:"Hao Chen".to_string(),age:44};
     let age: for<'a> fn(&'a Person) ->u8  = |p:&Person| p.age;
    let name: for<'a> fn(&'a Person) ->&'a String = |p:&Person| &p.name;
    println!("name={},age={}",name(&p),age(&p))
}
