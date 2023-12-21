//题目
// 使用两种方法来解决错误，不要新增代码行
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;
}

//答案
//法1
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s; //加上&
}
//法2
fn main() {
    let s = "hello, world".to_string();
    let s1: String = s; //变为String
}

//讲解
//法1
//s是String，s1是&str，所以加上&
//法2
//s是String，所以s1变为String
