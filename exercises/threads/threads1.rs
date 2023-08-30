// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.

// This program spawns multiple threads that each run for at least 250ms,
// and each thread returns how much time they took to complete.
// The program should wait until all the spawned threads have finished and
// should collect their return values into a vector.

// I AM DONE

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // handlers
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    // results
    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        // 可以通过将 thread::spawn 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。
        // thread::spawn 的返回值类型是 JoinHandle。
        // JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
