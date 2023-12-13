//题目
// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}

//答案
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; //去掉mut
    let r2 = &s; //去掉mut
    println!("{}, {}", r1, r2);
}

//讲解
//只能拥有要么一个可变引用, 要么任意多个不可变引用
//这里有两个，所以改为全部不可变
