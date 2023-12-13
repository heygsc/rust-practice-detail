//题目
// 修复错误
fn main() {
    let mut s = String::from("hello, ");
    borrow_object(s)
}
fn borrow_object(s: &String) {}

//答案
fn main() {
    let mut s = String::from("hello, ");
    borrow_object(&s) //加上&
}
fn borrow_object(s: &String) {}

//讲解
//borrow_object函数中，要求传入&String，是一个引用
//所以这里的s是String，需要加上&，变为&String
