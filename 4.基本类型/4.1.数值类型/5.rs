//题目
// 解决代码中的错误和 `panic`
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

//答案
// 解决代码中的错误和 `panic`
fn main() {
   let v1 = 247_u8 + 8; //251改为247
   let v2 = i8::checked_add(119, 8).unwrap(); //251改为119
   println!("{},{}",v1,v2);
}
//255 127

//讲解
//u8的最大值是255，255-8=247，所以改成247。
//i8的最大值是127，127-8=119，所以改成119。
