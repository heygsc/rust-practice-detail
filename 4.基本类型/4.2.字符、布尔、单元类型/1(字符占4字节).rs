//题目
// 修改2处 `assert_eq!` 让代码工作
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!")
} 

//答案
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); //1改为4

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); //3改为4

    println!("Success!")
} 
//Success!

//讲解
// 字符类型占用 4 个字节。
// 所以把1改为4，把3改为4。
