//题目
// 移除某个部分让代码工作
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // 这里 z 的类型是? 
}

//答案
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    
    let z = 10; // i32
}

//讲解
//x和y类型不同，这里删除了y后面的:u32