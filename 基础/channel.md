### rust 中的管道
```
use std::thread;
use std::sync::mpsc::channel;

fn main(){
    let (tx,rx) = channel();
    thread::spawn(move || {
        for i in 1..10{
            tx.send(i).unwrap();
        }
    });
    
    while let Ok(r) =rx.recv() {
        println!("received {}",r);
    }
}

```

### 多数据通道传输

```

use std::thread;
use std::sync::mpsc;

fn main(){
    // 发送方和接收方任一 通道数据
    let (sender,rescer) = mpsc::channel();
    thread::spawn(move || {
        let mut num =0;
        loop{
            //let val = String::from("hi"); 并发性
            sender.send(num).unwrap();// 返回一个Result<T,E> 的数据结构  unwarp 出错产生panic 当发送字符串的时候，数据的所有权
            // 发生了转移
              num +=1;
            if num >=100 {
                break;
            }
        }
   
    });

    let mut num:i32 =0;
    loop {
        let received = rescer.recv().unwrap();
        println!("{}",received);
        num +=1;
        if num >=100 {
            break;
        }
    }
}

```
