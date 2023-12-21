//题目
// 修复所有错误，并且不要新增代码行
fn main() {
    let  s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();
    println!("{}", s)
}

//答案
fn main() {
    let mut s = String::from("hello"); //加上mut
    s.push(',');
    s.push_str(" world"); //加上_str
    s += "!"; //删去to_string()
    println!("{}", s)
}

//讲解
//push和push_str都是在原有的字符串上追加，字符串必须是 mut 可变，所以加上mut
//push是追加字符，这里的 "world" 需要换成push_str
//push_str追加字符串字面量，to_string()转为了String，所以这里删去
