// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 将其设置为值为“Some”类型的if let语句
        if let Some(word) = optional_target {
            assert_eq!(word, target)
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 将此转换为 `while let` 语句 - 记住 `vector.pop` 也会增加一层 `Option<T>`。
        // 你可以将多个 `Option<T>` 嵌套到 `while let` 和 `if let` 中。
        while let Some(integer) = optional_integers.pop() {
            if let Some(integer) = integer {
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
