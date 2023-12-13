//题目
// 注释掉一行代码让它工作
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}",r1);
}

//答案
fn main() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}",r1); 
}

//讲解
//引用作用域的结束位置，是最后一次使用的位置
//r2是从定义到紧随其后的下一行
//r1是从定义到最后，因为最后的println使用了r1
//如果注释掉最后一行，r1的结束位置就变为，定义r2的上一行，因为r1.push_str是最后一次使用
