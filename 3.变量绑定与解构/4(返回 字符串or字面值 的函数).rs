//题目
// 修复错误
fn main() {
    println!("{}, world", x); 
}
fn define_x() {
    let x = "hello";
}

//说明
//首先，这道题放在这个位置，可能会让新手有点疑惑。因为这道题用到了后面的知识点。
//如果朋友们想跳过这题的话，是完全没问题的。可以等学了后面的知识点再回来看。
//官方目前放了两个解法，分别涉及了不同的知识点，这里分别讲解一下。

//解法一
//答案
fn main() {
    let x = define_x();//初始化
    println!("{}, world", x); 
}
fn define_x() -> String {
    let x = "hello".to_string();
    x//表达式，此处即为返回值
}
//hello, world

//讲解
// define_x 函数这边，->的意义，是返回值的类型，返回 String 。
//在函数体当中，第一行使用 to_string ，将它转为 String 。
//为什么要使用 to_string 呢？因为原来的类型是 &str (字符串字面值)，函数需要的返回值为 String 。
//使用 to_string 将 &str 转为 String 。
//函数体的最后一行，有一个孤零零的 x 。这个 x 的末尾没有分号，所以此处 rust 会认为这是表达式。
//在 rust 的函数中，函数的返回值，就是函数体中，最后一条表达式的返回值（当然我们也可以使用 return 提前返回）。
//所以这行孤零零的 x 就是函数的返回值，即String x 。

//解法二
//答案
fn main() {
    let x = define_x();
    println!("{:?}, world", x);
}
fn define_x() -> &'static str {
    let x = "hello";
    x
}
//"hello", world

//讲解
//这个解法和解法一的不同之处，在于输出中的:?和函数的定义。
//:?属于调试模式的，格式化输出，所以最终的输出中，hello 外面会出现双引号。
//函数的定义这边， &'static str 返回的就是 &str，所以不用 to_string 了。而 'static ，则是生命周期相关的。
