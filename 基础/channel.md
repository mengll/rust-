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
