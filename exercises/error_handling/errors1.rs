// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a hint.

// I AM DONE

// option 和 result 虽然都是enum的一种，但二者应用的场景却不同。
// option 主要用来表示一个可能存在或可能不存在的值，这点和Python的optional数据结构很相似。
// result 主要用来表示一个操作可能成功，也有可能失败。如果失败，则提供额外的失败信息。

// 借用 match 语句，二者可以进行某种程度的转化:
// 从Option到Result:
//      Some(v) => Ok(v), None => Err(e)
// 从Result到Option:
//      Ok(v) => Some(v), Err(e) => None
// 在实际场景下，二者常常搭配使用

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
