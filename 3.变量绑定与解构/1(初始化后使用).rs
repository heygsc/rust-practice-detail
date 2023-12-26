//题目
// 修复下面代码的错误并尽可能少的修改
fn main() {
    let x: i32; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}

//答案
fn main() {
    let x: i32 = 1; // 初始化为1
    let y: i32; 
    println!("x is equal to {}", x); 
}
//x is equal to 1

//讲解
// println 输出(宏使用 ! 调用)，用到了 x 。
// x 用 let 定义了，但未初始化。此处可以在 x 定义的同时初始化一个值。
