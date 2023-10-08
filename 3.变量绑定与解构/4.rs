// 修复错误
fn main() {
    println!("{}, world", x); 
}
fn define_x() {
    let x = "hello";
}

//讲解&答案
//首先，这道题放在这个位置，可能会让新手有点疑惑。因为这道题用到了后面的知识点。
//如果朋友们想跳过这题的话，是完全没问题的。可以等学了后面的知识点再回来看。
//官方目前放了两个解法，分别涉及了不同的知识点，这里分别讲解一下。

//解法一
//define_x函数这边，->的意义，是返回值的类型，返回String。
//在函数体当中，第一行使用to_string，将它转为String。
//为什么要使用to_string呢？因为原来的类型是&str，函数需要的返回值为String。
//使用to_string将&str转为String。
//函数体的最后一行，有一个孤零零的x。这个x的末尾没有分号，所以此处rust会认为这是表达式。
//在rust的函数中，函数的返回值，就是函数体中，最后一条表达式的返回值（当然我们也可以使用 return 提前返回）。
//所以这行孤零零的x就是函数的返回值，即字符串x。
fn main() {
    let x = define_x();//初始化
    println!("{}, world", x); 
}
fn define_x() -> String {
    let x = "hello".to_string();
    x//表达式，此处即为返回值
}
//hello, world

//解法二
//这个解法和解法一的不同之处，在于输出中的:?和函数的定义。
//:?属于格式化输出，所以最终的输出中，hello外面会出现双引号。
//函数的定义这边，&‘static str是返回了&str，所以不用to_string了。而'static，则是生命周期相关的。
fn main() {
    let x = define_x();
    println!("{:?}, world", x);
}
fn define_x() -> &'static str {
    let x = "hello";
    x
}
//"hello", world
