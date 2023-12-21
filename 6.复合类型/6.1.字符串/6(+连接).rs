//题目
// 修复所有错误，不要删除任何一行代码
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

//答案
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; //加上 clone 和 &
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

//讲解
//拼接会转移所有权，最后的println用到了s1，所以加上 clone 进行深拷贝，保留 s1 的所有权
//拼接是String和&str，所以加上 & 变为 &str
