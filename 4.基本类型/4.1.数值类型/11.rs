//题目
// 填空，并解决错误
fn main() {
    // 整数加法
    assert!(1u32 + 2 == __);

    // 整数减法
    assert!(1i32 - 2 == __);
    assert!(1u8 - 2 == -1);
    
    assert!(3 * 50 == __);

    assert!(9.6 / 3.2 == 3.0); // error ! 修改它让代码工作

    assert!(24 % 5 == __);
    
    // 逻辑与或非操作
    assert!(true && false == __);
    assert!(true || false == __);
    assert!(!true == __);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

//答案
// 填空，并解决错误
fn main() {
    assert!(1u32 + 2 == 3); // 1+2=3

    assert!(1i32 - 2 == -1); // 1-2 = -1
    assert!(1i8 - 2 == -1); //结果为-1，u8没有符号，应该转为i8有符号
    
    assert!(3 * 50 == 150); //3*50=150

    assert!(9 / 3 == 3); //浮点数计算转为二进制，这里改成整型

    assert!(24 % 5 == 4); //取模
    
    // 逻辑与或非操作
    assert!(true && false == false); //同时true才为true
    assert!(true || false == true); //有一个true即为true
    assert!(!true == false); 

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
// 0011 AND 0101 is 0001
// 0011 OR 0101 is 0111
// 0011 XOR 0101 is 0110
// 1 << 5 is 32
// 0x80 >> 2 is 0x20

//讲解
// 主要考察计算。
// 第三行，u8改为i8。
// 第五行，浮点数改为整型。
// 最后的println输出：
// AND表示&&，都为1才为1。
// OR表示||，两者有一个1，即为1。
// XOR表示异或，相同为0，不同为1。
// <<表示左移，乘2，<<5即为2的5次方，1*(2^5)=32
// >>表示右移，除2，>>2，2^2=4，除以4。虽然Ox是十六进制，这里不影响。80/4=20。
