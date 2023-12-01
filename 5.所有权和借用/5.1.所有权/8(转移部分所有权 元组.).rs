//题目
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // 仅修改下面这行代码，且不要使用 `_s`
   println!("{:?}", t);
}

//答案
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1); // t.1
    // "world"
}

//讲解
//用 . 访问元组
//这里 _s 拿走了元组的，前半部分，"hello"，t.0
//此时后半部分的所有权，仍在 t 中，所以可以使用 t.1
// :? 属于格式化输出
