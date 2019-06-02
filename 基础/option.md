### opion 是rust中的枚举变量 表示对象的 空对象，

```
fn show() ->i32{
    31
}

fn main(){
    let x :Option<i32> = None;
    println!("{:?}",x);
    let x  = Some(4);
    match x {
        Some(_) => println!("Had value"),
        None => println!("Not Found the Value")
    }

    let x = Some(show); // option 是任意的类型，表示有值得情况下数据，
    let mm = x.unwrap()();
    println!("{}",mm);
}


```
