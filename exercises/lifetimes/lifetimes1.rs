// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a hint.

// I AM DONE

// 1.
// 生命周期参数的机制与其说是一种约束，不如说是一种约定。
// 即，该函数会返回某种“其生命周期可以被【比较】”的结果。
// 在本例中，这意味着返回值的生命周期，和输入的x或y的至少一个的生命周期可以被比较。

// 2.
// 其中，函数名后面<'a>表示的是该函数输入值的可用的生命周期的集合。
// 在本例中，则表示输入的参数都会用一种生命周期参数来表示，即'a。
fn longest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}


