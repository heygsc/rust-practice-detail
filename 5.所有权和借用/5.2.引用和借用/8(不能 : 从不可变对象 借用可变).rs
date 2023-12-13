//题目
fn main() {
    // 通过修改下面一行代码来修复错误
    let  s = String::from("hello, ");
    borrow_object(&mut s)
}
fn borrow_object(s: &mut String) {}

//答案
fn main() {
    let mut s = String::from("hello, "); //加上mut
    borrow_object(&mut s)
}
fn borrow_object(s: &mut String) {}

//讲解
//s此时是不可变，引用了可变，是不可以的
//所以加上mut，让s可变
