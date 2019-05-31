### rus值的作用域，和返回值的数据转
```
fn main(){
    let arr:[i32;5] = [1,2,3,4,5];
    for i in 1..5 {
        println!("超出长度==》{}",arr[i]); // 市场报错的
    }
    {
        //println!("未声明-》{}",s); // 作用域的变量必须先声明再使用，不然会报错
        let s = "hello";
        println!("后声明=》{}",s);
    }

    let x  =5;
    let y = x;
    println!("x->{:p}--y->{:p}",&x,&y);

    let names = "mll";
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} , world",s1);
    println!{"s1:{:p} ,s2:{:p}",&s1,&s2}

    let s = String::from("hello");
    takes_ownership(s); // 引用类型，输出当前的值的操作 感觉像是传递的是引用
    //println!("{}",s); // 调用了函数之后会调用一次drop移除了当前的变量  可变数据是
    
    let xm = 5;
    make_copy(xm); // 像golang 中调用了值的复制
    println!("数值类型 =》{}", xm);

    // 1 Rust 中的每一个值都有一个被称为其所有者的变量
    // 2 值有且只有一个所有者
    // 3 当所有者（变量）离开作用域，这个值被丢弃
    // 字符串字面量，分配到栈区
    let s = "hello";
    takes_ownership(s.to_owned());
    println!("--enders-=s-{}",s);

    // 函数的返回值，作用域 转移返回值的所有权限
    // 作用域中的数据离开作用域时候，值将通过drop的方式清理掉，除非数据被移动为另一个变量所有

    let  s1 = String::from("helo");
    let (s2,len) = calculate_length(s1);  // 元组的析构
    println!("===strlen==>{}--- {}",s2,len);
}
fn calculate_length(s:String) ->(String,usize){
    let length =s.len();
    (s,length)
}

fn takes_ownership(some_string:String){ 
    println!("内容--》{}",some_string); // 移除作用域并调用drop方法，占用的内存被释放
}

fn make_copy(some_integer :i32) {
    println!("数值内容被copy=》{}",some_integer);  
}

fn gives_ownership()->String {
    let some_string = String::from("hello"); // 进入some_string 的作用域中
    some_string   // 返回值 所有权移交给调用他的方法
}

// 传递数组返回值
fn takes_and_gives_back(a_string :String) ->String { // 进入a_string 的作用域中
    a_string            // 返回当前的a_string 到调用者
}


```
