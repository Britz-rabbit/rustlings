// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

// I AM DONE


// 在 Rust 中,trait 里定义的方法是默认采用 self 作为参数的,这表示trait方法会获取类型的所有权。
trait AppendBar {
    fn append_bar(self) -> Self;
}

// 但是在为具体类型实现trait时,可以选择是否获取所有权,这由实现来决定。
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
