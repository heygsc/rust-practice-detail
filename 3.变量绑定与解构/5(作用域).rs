//题目
// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }
    assert_eq!(x, 12);
    let x = 42;
    println!("{}", x); // 输出 "42".
}

//答案
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); //改为12
    }
    assert_eq!(x, 5); //改为5
    let x = 42;
    println!("{}", x);
}
//42

//讲解
//外层 x 是5，内层 x 是12。
//这里内层的改变只影响了内层。
