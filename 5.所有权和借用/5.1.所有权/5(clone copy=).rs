//题目
// 不要使用 clone，使用 copy 的方式替代
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

//答案
fn main() {
    let x = (1, 2, (), "hello"); //删去 to_string()
    let y = x; //删去 clone
    println!("{:?}, {:?}", x, y);
}

//讲解
//不使用clone，所以删去 clone
//此时存在 to_string()，将 &str 转为了 String
// &str 是存在栈上的，String 存在堆上
//要求用 copy 替代，所以需要改为栈上，删去 to_string()
