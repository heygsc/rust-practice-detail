//题目
// 修复错误
fn main() {
    let mut s = String::from("hello, ");
    push_str(s)
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

//答案
fn main() {
    let mut s = String::from("hello, ");
    push_str(&mut s) //加上&mut
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

//讲解
//push_str要求传入可变引用&mut String，s是可变的String
//所以加上&mut，变为可变的引用
