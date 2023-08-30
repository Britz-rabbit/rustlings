// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

// I AM DONE

use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

// 1.Arc<T>:原子引用计数器，允许在多线程之间安全地共享某个值。
// 2.Mutex<T>:互斥器，允许在多线程之间安全地修改某个值。
// 3.Arc<Mutex<T>>:将Arc和Mutex结合起来，使得一个值可以在多线程之间安全地共享和修改。
// 4.Mutex<Arc<T>>:一般不用。

// 起了10个线程，在每个线程里获取值，然后修改。因此应当使用Arc<Mutex<T>>。
fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared: Arc<Mutex<JobStatus>> = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250)); //注释掉这一行的话基本就只会打印出9和10了
            // Arc和Mutex都支持自动deref，因此直接对Arc<Mutex<JobStatus>>类型的数据进行lock()操作也是可以的
            // lock().unwrap()的结果是MutexGuard类型的
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 每个线程结束再进行下一个线程，保证数据的互斥访问和修改
    for handle in handles {
        // 子线程的join操作会堵塞主线程，但是不影响其他子线程的操作
        // 注释掉下面一行，打印结果会全部为0。不注释的话，打印结果也是并不是稳定的1到10(多线程之间竞争修改,而数据是共享的)。
        handle.join().unwrap();

        let result = status.lock().unwrap();
        println!("jobs completed {}", result.jobs_completed);
    }
}
