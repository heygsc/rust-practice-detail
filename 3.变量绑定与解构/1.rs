// 修复下面代码的错误并尽可能少的修改
fn main() {
    let x: i32; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}

//讲解&答案
//println输出，用到了x。
//x用let定义了，但未初始化。此处可以在x定义的同时初始化一个值。
fn main() {
    let x: i32 = 1; // 初始化
    let y: i32; 
    println!("x is equal to {}", x); 
}

//x is equal to 1
