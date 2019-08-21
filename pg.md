### rust 连接数据库如果账户中，出现如 ！或#，或& 等特殊符号的时候 要使用 urldecode 抓马 Wuaf%21%40%23com
```
extern crate postgres;
use postgres::{Connection,TlsMode};
fn main(){
    let conn = Connection::connect("postgres://tianyan:Wuaf%21%40%23com@192.168.1.240:45/ti", TlsMode::None).unwrap();
    println!("{}",conn.is_active());

    for row in &conn.query("SELECT id, muid FROM ty_device_active_new limit 10", &[]).unwrap() {
        let id:i32 = row.get(0);
        println!("Found person {:?}", id);
    }
}

```
