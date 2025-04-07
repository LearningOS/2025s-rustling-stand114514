// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// 这个练习使用了一些我们将在课程稍后才会学到的概念，比如 `Box` 和 `From` 特性。
// 现在理解它们的细节并不重要，但如果你愿意，可以提前阅读相关内容。
// 目前，你可以将 `Box<dyn ???>` 类型视为一种“我想要任何能够执行 ??? 的东西”的类型，
// 鉴于 Rust 通常对运行时安全性的严格标准，这可能会让你觉得有些宽松！
//
// 简而言之，这里提到的使用 `Box` 的特定用例适用于当你想要拥有一个值，并且只关心它是否实现了某个特定的特质（trait）时。
// 为此，`Box` 被声明为 `Box<dyn Trait>` 类型，其中 `Trait` 是编译器在该上下文中查找的任何值上所要求的特质。
// 在这个练习中，这个上下文是指在 `Result` 中可能返回的潜在错误。
//
// 我们可以用什么来描述这两个错误？换句话说，有什么特点吗
// 这两个错误实现了什么？
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
