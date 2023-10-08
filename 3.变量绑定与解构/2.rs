// 完形填空，让代码编译
fn main() {
    let __ = 1;
    __ += 2; 
    
    println!("x = {}", x); 
}

//讲解&答案
//println输出，用到了x，所以应该提前定义并初始化x的值。
//rust中，默认一个值是不可变的。所以如果想要x+=2，需要加一个mut(mutable，本意为可变的)。
fn main() {
    let mut x = 1;//定义，初始化，mut可变
    x += 2; //mut，可以变
    
    println!("x = {}", x); 
}
//x = 3
