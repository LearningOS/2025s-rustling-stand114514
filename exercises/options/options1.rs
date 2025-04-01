// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// 这个函数返回冰箱里还剩下多少冰淇淋。
// 如果是在晚上10点之前，还剩下5块。到了晚上10点，有人把它们都吃掉了，
// 所以之后就一块也没有了:(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 23 {
        None 
    } else {
        Some(if time_of_day >= 22 { 0 } else { 5 }) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: 修复这个测试。你该如何获取其中包含的值呢？
        // Option?
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
