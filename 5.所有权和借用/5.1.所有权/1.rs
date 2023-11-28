//题目
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}

//答案
//法1
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y); //hello, world,hello, world
}
//法2
fn main() {
    let x = String::from("hello, world");
    let y = x.as_str();
    println!("{},{}",x,y);
}
//法3
fn main() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
//法4
fn main() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}

//讲解
//把所有权从 x 转移给了 y
//法1 法2 : 通过 clone 和 as_str 解决
//法3 : x 没有所有权 拷贝的只是引用
//法4 : x 是字符串字面值
