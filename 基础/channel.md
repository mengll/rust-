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


### 多生产者的模式 单生产者
```
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main(){
 let (tx,rx) = mpsc::channel();
 let tx1 = mpsc::Sender::clone(&tx); // 通过复制的方式创建多个所有者，
 
 thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread")
    ];
    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1)); // 延迟1s
    }
 });

 thread::spawn(move || {
     let vals = vec![
         String::from("more"),
         String::from("message"),
         String::from("for"),
         String::from("you")
     ];
     for val in vals {
         tx.send(val).unwrap();
         thread::sleep(Duration::from_secs(1));
     }
 });

 for received in rx{
     println!("Got:{}",received);
 } // 并行的
}
```


