//题目
//修改一行代码以通过编译
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x; 
    x += 3;
    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
}

//答案
fn main() {
    let mut x: i32 = 1;
    x = 7;
    let mut x = x; //设为可变
    x += 3;
    let y = 4;
    let y = "I can also be bound to text!"; 
}

//讲解
//第一行 x 可变，所以后面 x=7 没问题。
// x=x，变量遮蔽，变为默认的不可变。
//需要加上 mut，才可以 x+=3 。
// y 变量遮蔽，没有问题。
