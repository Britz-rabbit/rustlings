// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[test]
fn slice_out_of_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b  = a.split(|num| num % 3 == 0);//Split is an iterator
    // for i in b{
    //     println!("{:?}",i);
    // }

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
