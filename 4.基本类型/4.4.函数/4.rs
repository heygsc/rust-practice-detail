//题目
fn main() {
    println!("Success!");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}
// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    
}

//答案
//法1
fn never_return_fn() -> ! {
    panic!()
}
//法2
fn never_return_fn() -> ! {
    loop{        
    }
}
//法3
fn never_return_fn() -> ! {
    unimplemented!()
}
//法4
fn never_return_fn() -> ! {
    todo!()
}

//讲解
//和上一题很像，实现 never_return
//法1 : panic!() 崩溃
//法2 : loop 无限循环
//法3 : unimplemented 说明代码还没实现，执行到这里会发生 panic
//法4 : todo 和法3的 unimplemented 类似
