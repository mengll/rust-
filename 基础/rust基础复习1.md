### 主要涉及了，结构结构 和 match 匹配的相关的操作
```
fn main(){
    let s = format!("{1}是个有着{0:>width$}KG重 ，{height:?}cm",81,"wayslog",width=4,height=178);
    println!("{}",s);
    
    let tup =(0u8,1u8);
    let (x,y) = tup;
    println!("x:{},y:{}",x,y);
   let c = 'c';
   let x = 1;
    match c { 
        x => println!("x:{},y:{}",x,c), // match 解构被赋值为c
    }

    println!("x:{}",x); 

    // 更强大的解构函数 match 
    struct Point {
        x:i64,
        y:i64,
    }
    let point = Point{x:0,y:0};
    match point {
        Point{x:x1,y:y1} => println!("{},{}",x1,y1), // match 结构解构
    }

    match point {
        Point{x,..} => println!("{}",x),  // 解构其中的部分数据
    }

    let m = 1;
    match m {
        1 | 2 => println!("匹配-》"),
        _ => println!("default") // 数据匹配的时候，必须要存在默认值，如果是内容结构就不是必须的存在默认值
    }

    let mut xx = 5;
    match xx {
        ref mut mr => xx+=1,
    }
    println!("xx:{}",xx); // 

    // 模式匹配的后置条件

    let xxy = 4;
    let yy = false;
    // if表达式的可以放在match的模式的后面，
    match xxy {
        4 | 5 if yy => println!("yesy"),
        _ => println!("no")
    }
    
}
```
