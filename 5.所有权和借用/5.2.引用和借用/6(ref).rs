//题目
fn main() {
    let c = '中';
    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let __ r2 = c;
    assert_eq!(*r1, *r2);
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
}
// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//答案
fn main() {
    let c = '中';
    let r1 = &c;
    let ref r2 = c; // ref
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1),get_addr(r2));
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//讲解
//*解引用，get_addr传入&char，所以r1和r2是引用
//这里用ref作为引用
