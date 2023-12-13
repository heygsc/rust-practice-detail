//题目
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
}

//答案
fn main() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push_str("world");
}

//讲解
//这里r1和r2都是可变引用
//使用r1报错:可变引用次数过多
