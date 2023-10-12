//题目
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!")
}

//答案
fn main() {
    let f = true;
    let t = true || false; // && 改为 ||
    assert_eq!(t, f);

    println!("Success!")
}

//讲解
// &&是同时满足，两个true才为true。
// ||，两个里有一个true，即为true。
// f是true，为了让t和f相等，应该让t也是true。
// 所以这里 && 改为 || 即可。
