// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

// I AM DONE

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    // ? 操作符只能用于result类型的返回值，会自动取出成功的Ok操作的值，或者中断函数并返回err类型的值
    // 故使用 ? 操作符则必须考虑到result类型中err的返回值处理
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    };

    // 注意：ok()和Err()操作是和Result返回值对应的
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
