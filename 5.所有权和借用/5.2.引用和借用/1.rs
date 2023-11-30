//题目
fn main() {
   let x = 5;
   // 填写空白处
   let p = __;

   println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

//答案
fn main() {
   let x = 5;
   let p = &x;
   println!("x 的内存地址是 {:p}", p); 
}

//讲解
//题目要返回的是地址
//这里 & 表示引用
