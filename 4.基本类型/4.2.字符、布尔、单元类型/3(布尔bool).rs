//题目
// 使成功打印
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!")
    }
} 

//答案
fn main() {
    let _f: bool = false;

    let t = false; // true改为false
    if !t {
        println!("Success!")
    }
} 
//Success!

//讲解
// !t 让true变为false，false变为true。
// 这里希望打印，!t 为 true ，所以把 t 改为 false。
