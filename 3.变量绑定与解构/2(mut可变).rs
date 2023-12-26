//题目
// 完形填空，让代码编译
fn main() {
    let __ = 1;
    __ += 2; 
    
    println!("x = {}", x); 
}

//答案
fn main() {
    let mut x = 1;//定义，初始化，mut 可变
    x += 2; // mut，可以变
    
    println!("x = {}", x); 
}
//x = 3

//讲解
// println 输出，用到了 x ，所以应该提前定义并初始化x的值。
// rust 中，默认一个变量是不可变的。所以如果想要 x+=2 ，需要加一个 mut ( mutable ，本意为可变的)。
