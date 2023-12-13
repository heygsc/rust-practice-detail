// 下面的代码没有任何错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");
}

fn borrow_object(s: &String) {}

//此题给出的，为正确的示例代码，无需解答
