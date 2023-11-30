//题目
fn main() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, y);
}

//答案
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // 加上 *
}

//讲解
// y 是引用
//比较的是 5 这个值，可以用 * 解引用
//这样 *y 就是 5 这个值
