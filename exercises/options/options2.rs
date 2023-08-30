// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
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

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`s into while let and if let

        // 注意：while let算loop和match的语法糖。如下所示：

        // loop {
        //     match optional_integers.pop() {
        //         Some(integer) => {
        //             assert_eq!(integer, Some(cursor));
        //             cursor -= 1;
        //         }
        //         _ => break;
        //     }
        // }

        // 第一种写法的问题在于，integer被判断为Option<i8>类型的值，因此integer会成功判断并被赋值为None
        // while let Some(integer) = optional_integers.pop() {
        //     assert_eq!(integer, Some(cursor));
        //     cursor -= 1;
        // }

        // 第二种写法则成功解决了这个问题，因为这里的integer是i8类型的值，因此不会被赋值为None
        // while let Some(integer) = optional_integers.pop().flatten() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        // 如果你喜欢的话，你也可以这么写
        while let Some(integer) = optional_integers.pop().unwrap() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
