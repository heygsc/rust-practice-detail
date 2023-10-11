//题目
// 填空
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..__), Range{ start: 1, end: 5 });
    assert_eq!((1..__), RangeInclusive::new(1, 5));
}

//答案
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

//讲解
// 第一个是不包含5，第二个包含末尾的5。
// 关于Range和RangeInclusive，最权威的可以去看官方文档，这里已经准备好了具体链接：
// Range：https://doc.rust-lang.org/std/ops/struct.Range.html
// RangeInclusive：https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
