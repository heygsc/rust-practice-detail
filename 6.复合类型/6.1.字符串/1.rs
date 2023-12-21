//题目
// 修复错误，不要新增代码行
fn main() {
    let s: str = "hello, world";
}

//答案
fn main() {
    let s: &str = "hello, world"; //加上&
}

//讲解
//"hello, world"是字面量，类型为&str，而不是str
//所以前面加上&
