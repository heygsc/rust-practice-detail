//题目
// 修复错误，不要删除任何代码行
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

//答案
//法1
fn main() {
    let s = String::from("hello, world");
    print_str(s.clone()); // clone()
    println!("{}", s);
}
fn print_str(s: String)  {
    println!("{}",s)
}
//hello, world
//hello, world

//法2
fn main() {
    let s = String::from("hello, world");
    print_str(&s); // 加上 &
    println!("{}", s);
}
fn print_str(s: &String)  { // 加上 &
    println!("{}",s)
}

//讲解
//题目中，s 的所有权被转移到 print_str,所以后面的 println! 不生效，拿不到 s
//法1 : clone() 深拷贝，这里是一个新的s，后面可以用原来的 s
//法2 : 加上&，即为 &str，字面量字面值，不会转移所有权
