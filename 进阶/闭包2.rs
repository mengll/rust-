fn applay<F>(f:F) where F:FnOnce() {
    f();
}

fn applay_to_3<F>(f:F)->i32 where F:Fn(i32) ->i32{
    f(3)
}

// 转移所有权的实现
fn move_func<F>(f:F) where F:FnOnce(){
    f()
}

fn main(){
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = ||{
        println!("T said {}.",greeting);

        farewell.push_str("||||");

        println!("Then i screamed {}", farewell);

        println!("Now i can sleep ,zzzz");

        mem::drop(farewell);
    };

    applay(diary);

    let double = |x| 2 *x;

    println!("3 double:{}",applay_to_3(double))  ;
    println!("{}",greeting)    ;


    let mut c = 23;
    let mut s = "deada".to_string();
    // 当显示的使用move 关键字的时候，会发生所有权发生转移报错
    move_func( ||{
         c = c + 12;
         println!("The ender {}",c);
         println!("{}",s);
         // 限定了发FNone 最终的变量的捕获的方式是不确定的
         // mem::drop(s) 如果未显示的使用move的操作，则变量只是通过引用的方式传递，可能是可变引用，也
         // 可能是不可变引用
        s.push_str("ppp");
    })  ;
    println!("This is {}",c);  // 确定最终的捕获变量的操作
    println!("{}",s)
    
}
