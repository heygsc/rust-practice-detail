//题目
fn main() {
   let t = (String::from("hello"), String::from("world"));

   // 填空，不要修改其它代码
   let (__, __) = __;

   println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

//答案
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone(); // s1 s2 t.clone()
    println!("{:?}, {:?}, {:?}", s1, s2, t); // "hello", "world", ("hello", "world")
}

//讲解
//输出 s1 s2 t
// t 的所有权还在，所以需要 s1 s2 有 t 的值，并且没有转移所有权
//这里用 clone 进行深拷贝，s1 s2拿到了值，并且保留了 t 的所有权
