//题目
// 填空
fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
}

//答案
fn main() {
    let mut s = "".to_string();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
}

//讲解
//push_str 和 push 都是追加到 String 类型的末尾
//由最后一行可以推测，push_str 之前，是空的
//所以初始化一个""，空的String
