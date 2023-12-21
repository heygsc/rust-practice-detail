//题目
// 使用至少两种方法来修复错误
fn main() {
    let s = "hello, world";
    greetings(s)
}
fn greetings(s: String) {
    println!("{}",s)
}

//答案
//法1
fn main() {
    let s = "hello, world";
    greetings(s.to_string()) //加上to_string
}
fn greetings(s: String) {
    println!("{}",s)
}
//法2
fn main() {
    let s = "hello, world";
    greetings(String::from(s)) //String
}
fn greetings(s: String) {
    println!("{}",s)
}

//讲解
//greetings需要String类型，s为&str，所以需要将&str转为String
//to_string和String::from都可以将&str转为String
