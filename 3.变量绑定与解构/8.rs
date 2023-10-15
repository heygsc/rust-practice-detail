//题目
// 修复下面代码的错误并尽可能少的修改
fn main() {
    let (x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

//答案
fn main() {
    let (mut x, y) = (1, 2);  //添加了mut，使x可变
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

//讲解
//加上mut使x可变，x+=2生效，从而x变为3
