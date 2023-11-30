//题目
fn main() {
   let v = (let x = 3);
   assert!(v == 3);
}

//答案
fn main() {
    let v = {
        let x = 3;
        x  //x作为返回值
    };
    assert!(v == 3);
}

// 讲解
// let是语句，不能赋给其它值。
// 这里把x作为最后一个表达式，作为返回值，也就是3。
// 所以v等于3。
