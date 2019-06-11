# rust
rust 学习笔记

```
'a 生命周期提示符，前面使用' 标识 
简单的数据类型实现了 copy的功能 复杂的数据

常见的 数字类型，bool类型 共享借用指针& 都是具有copy 属性的的类型 而box vec 可借用指针&mut等类型不具备Copy属性的类型 
数组类型，如果内部的元素类型是Copy类型 这个数组也是copy 类型=
struct 和  enum 类型不会自动实现 Copy trait 只有当内部成员是Copy类型 才允许实现Copy的类型
``` 
