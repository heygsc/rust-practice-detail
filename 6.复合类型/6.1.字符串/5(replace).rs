//题目
// 填空
fn main() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.__("dogs", "cats");
    assert_eq!(s1, "I like cats")
}

//答案
fn main() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats"); //replace
    assert_eq!(s1, "I like cats")
}

//讲解
//replace进行替换，这里dogs换成cats
