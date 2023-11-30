//题目
//两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}

//答案
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5); // -3改为-5

    for c in 'a'..='z' {
        //加上as u8
        println!("{}",c as u8);
    }
}

//讲解
// -3..2不包括2。
// -3 + -2 + -1 + 0 + 1 = -5
// 所以-3改为-5
// 大写字母“A”到“Z”的ASCII码值为65到90。
// 小写字母“a”到“z”的ASCII码值为97到122。
// 加上as u8，把字母转成数字，即为ASCII码。a-z正好是97-122。
