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

//讲解&答案
//外层x是5，内层x是12。
//这里内层的改变只影响了内层。
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x); // 输出 "42".
}
//42
