//题目
// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}
fn greetings(s: &str) {
    println!("{}",s)
}

//答案
//法1
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s) //加上&
}
fn greetings(s: &str) {
    println!("{}",s)
}
//法2
fn main() {
    let s: Box<&str> = "hello, world".into(); //加上&
    greetings(*s) //加上*
}
fn greetings(s: &str) {
    println!("{}",s)
}

//讲解
//法1
//Box将值存在堆上
//这里加上&，有一个解引用，Box的deref，会让&Box变为&T
//这里T就是str，即变为&str
//如题目所说，&可以用来将 Box<str> 转换为 &str 类型，原因就在于此
//&str正是greetings所需要的

//法2
//Box是在堆中，可以用 * 解开Box，变回Box<T>中的T类型
//初始的T是str，这里再加上&，T即为&str
