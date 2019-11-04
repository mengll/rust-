### 如果泛型增加了，较多的trait的 限定 代码可能变得不太易读
  fn foo<T:A,K:B+C,R:D>(a:T,b:K,c:R) {.....} // 复杂的实现
Rust 提供了where 关键字 用来对当前的，情况进行重构
foo<T,K,R> where T:A,K:B+C,R:D{}
