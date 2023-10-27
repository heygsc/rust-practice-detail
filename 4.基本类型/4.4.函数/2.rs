//题目
fn main() {
   print();
}
// 使用另一个类型来替代 i32
fn print() -> i32 {
   println!("hello,world");
}

//答案
fn main() {
   print();
}
fn print() -> () {  //i32改为()
   println!("hello,world");
}
//hello,world

//讲解
//分号结尾，不是表达式。函数返回单元类型()，代表无返回值。
