//题目
//修复编译器输出的 warning，不可以移除 let x = 1 所在的代码行
fn main() {
    let x = 1; 
}
// compiler warning: unused variable: `x`

//答案
fn main() {
    let _x = 1; 
}

//讲解
//下划线作为变量名的开头，就不会警告未使用的变量。
