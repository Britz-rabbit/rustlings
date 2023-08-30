// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

// I AM DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![]; // vec![] can be saw as String
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!

            let value = match command {
                Command::Uppercase => string.to_string().to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(count) => {
                    // make &string string, then operate it
                    let mut result = string.to_string();
                    for _ in 0..*count {
                        // 方式一，调用push_str()
                        // push_str() 接受一个&str，并将其连接在原本string的最后方
                        // result.push_str("bar");

                        // 方式二，使用clone加add操作（并没有通过编译）
                        // 调用add方法会消耗掉self: pub fn add(self, rhs: Rhs) -> Self::Output
                        // clone()是深拷贝,会创建了一个新的result副本，对副本进行操作.但这副本的值实际上并没有被用上
                        // result.clone() + "bar"; 

                        // 方式三，使用add直接操作原本的result空间
                        // 如果采用这种写法，则可以操作到正确的内存空间，并通过编译
                        result += "bar";
                    }
                    result
                }
            };


            output.push(value)

        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);

        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
