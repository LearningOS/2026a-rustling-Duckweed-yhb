// variables4.rs
//
// Make me compile! `x` is reassigned later, so it needs to be declared as a
// mutable binding.
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
