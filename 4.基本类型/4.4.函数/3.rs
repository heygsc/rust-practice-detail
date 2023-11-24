//题目
// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
  
}

//答案
//解法1
fn main() {
    never_return();
}

fn never_return() -> ! {
    panic!("never_return");
}

//解法2
fn main() {
    never_return();
}

fn never_return() -> ! {
    loop{
        println!("never_return");
    }
}

//讲解
//!用作函数返回类型，表示永不返回
//可以用作导致程序崩溃，永不跳出的无限循环
//解法1:panic崩溃
//解法2:loop表示无限循环
