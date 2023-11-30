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
//法1 : 通过 clone() 进行了深拷贝
//法2 : as_str() 是 &str 
//法3 : & 会自动解引用为 &str
//法4 : 这里的 x 改为了字符串字面值( &str )
