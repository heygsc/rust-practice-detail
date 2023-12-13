//题目
fn main() {
    let mut s = String::from("hello, ");
    // 填写空白处，让代码工作
    let p = __;
    p.push_str("world");
}

//答案
fn main() {
    let mut s = String::from("hello, ");
    let p = &mut s; //&mut s
    p.push_str("world");
}

//讲解
//当前s本身可以push_str，或者mut p = s也可以
//这里要求填写空白处
//&mut s，是s的可变引用
