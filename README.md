# rust
rust 学习笔记

```
'a 生命周期提示符，前面使用' 标识 
简单的数据类型实现了 copy的功能 复杂的数据

常见的 数字类型，bool类型 共享借用指针& 都是具有copy 属性的的类型 而box vec 可借用指针&mut等类型不具备Copy属性的类型 
数组类型，如果内部的元素类型是Copy类型 这个数组也是copy 类型=
struct 和  enum 类型不会自动实现 Copy trait 只有当内部成员是Copy类型 才允许实现Copy的类型

包引入其他的应用包
pub use crate::db::afmysql::mysql; 本项目下

闭包与匿名函数区别
 闭包是获取上下文变量的匿名函数 Fn|| ->i32 {x+1}
 匿名函数 |a|->i32 { a+1}

设置为开发
rustup default nightly
``` 

### 配置代理
设置环境变量 RUSTUP_DIST_SERVER(用于更新 toolchain)
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
以及 RUSTUP_UPDATE_ROOT(用于更新 rustup)
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

### ziyuan 
https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html

https://stevenbai.top/rust/my_note/
