//题目
// 填空，让代码工作
fn main() {
    assert_eq!(i8::MAX, __); 
    assert_eq!(u8::MAX, __); 
}

//答案
// 填空，让代码工作
fn main() {
    assert_eq!(i8::MAX, 127);  // i8最大值
    assert_eq!(u8::MAX, 255);  // u8最大值
}

//讲解
//i是integer，有正负号。u是undesigned，无符号。
// i : -2^(n-1) ~ 2^(n-1)-1
// u : 0 ~ 2^n-1
//i8的MAX，最大值即为2^(8-1)-1=2^7-1=128-1=127
//u8的MAX，最大值即为2^8-1=256-1=255
