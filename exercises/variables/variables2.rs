// variables2.rs
//
// Make me compile! The compiler cannot infer the type of the variable `x`.
// Give `x` an explicit type annotation.
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x: i32 = 10.into(); // 编译器无法推断 x 的类型，请给它加上显式类型标注
    println!("x is {}", x);
}
