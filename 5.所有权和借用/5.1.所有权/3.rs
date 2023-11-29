//题目
fn main() {
    let s = give_ownership();
    println!("{}", s);
}
// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.into_bytes();
    s
}

//答案
//法1
fn give_ownership() -> String {
    let s = String::from("hello, world");
    let _s = s.as_bytes(); // into 换成 as
    s
}
//法2
fn give_ownership() -> String {
    let s = String::from("hello, world"); //删去原来下面那行
    s
}

//讲解
//法1 将 into_bytes() 换为 as_bytes(),因为 into_bytes 是 self,转移了所有权。而 as_bytes() 是 &self
//法2 直接删去 into_bytes
