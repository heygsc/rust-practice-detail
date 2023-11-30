//题目
fn main() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let s1 = s;

    s1.push_str("world")
}

//答案
fn main() {
    let s = String::from("hello, ");
    let mut s1 = s; //加上 mut
    s1.push_str("world")
}

//讲解
// s1 不可变，前面加上 mut
