//题目
fn main() {
    // 填空
    let b = __;
    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };
    println!("Exercise Failed if printing out this line!");
}

//答案
fn main() {
    let b = false; // false
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };
    println!("Exercise Failed if printing out this line!");
}

//讲解
// match 模式匹配，true 的话，直接执行到最后的 println failed
// false 会走到success，panic 崩溃，不会执行到最后的failed
//所以这里填上 false
