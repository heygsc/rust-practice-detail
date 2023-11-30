//题目
// 填空
fn main() {
    let v: u16 = 38_u8 as __;
}

//答案
fn main() {
    let v: u16 = 38_u8 as u16;  //u16，和v一致
}

//讲解
//as可以类型转换，v是u16，这里把38换成同样的u16。
