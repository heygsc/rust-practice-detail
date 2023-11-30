//题目
// 修改一行让代码正常打印
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

//答案
fn main() {
    let c1 = '中';//双引号改成单引号
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
//中

//讲解
//单引号''对应 字符（char）
//双引号""对应 字符串（String）
//这里是字符，所以双引号改成单引号。
