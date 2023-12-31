//题目
// 修改 `assert!` 让代码工作
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);
}

//答案
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); //1579改为1597
}

//讲解
// 如果数值较长，可以在任意位置处使用下划线_划分，增加可读性。
// 0b是二进制（b是binary）
// 0o是八进制（o是octal）
// 0x是十六进制（x是hexadecimal）
// 0xff=16*15+15=255
// 0o77=7*8+7=63
// 0b1111_1111=1+2+4+8+16+32+64+128=255
// v=1024+255+63+255=1597
// 所以把1579改成1597
