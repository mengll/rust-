//! 整个包的描述
/// 闭包test 文档注释 cargo doc --open 创建打开文档
/// #exapmle
/// ```
/// let mut func  = cacheone();  // 原始样式输出 执行代码测试  cargo test
/// let one= func()；
/// assert_eq(1,one);
/// ```
pub fn  cacheone() ->impl FnMut()-> i32 {
    let mut stemp = 0;
     move ||->i32 {
        stemp = stemp +1;
         stemp
    }
}

fn main(){
    let mut func = cacheone();
    let  one = func();
    let  two = func();
//    let  three = func();
    println!("{},->{}",one,two); // 三者
}
