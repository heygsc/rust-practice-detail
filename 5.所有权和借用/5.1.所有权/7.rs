//题目
fn main() {
    let x = Box::new(5);
    
    let ...      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}

//答案
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(1);     // mut y
    
    *y = 4;    
    assert_eq!(*x, 5);
}

//讲解
// x 正常
//题目直接用了 *y，所以需要提前定义 y，并使其 mut 可变
