//题目
// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

//答案
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit()); // v改为_v

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

//讲解
// implicitly_ret_unit函数中，println!() 的返回值，是单元类型 ()。
// v是(2,3)，_v是()。
// _v才是单元类型。
// 所以把v改为_v。
