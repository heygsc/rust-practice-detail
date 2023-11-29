//题目
// 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// 只能修改下面的代码!
fn take_ownership(s: String) {
    println!("{}", s);
}

//答案
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2); // hello, world
}
fn take_ownership(s: String) -> String {
    println!("{}", s); // hello, world
    s
}

//讲解
// println 这里是语句，没有返回
//用 s 作为返回，并加上 String 类型
