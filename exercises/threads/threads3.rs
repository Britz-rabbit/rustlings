// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.

// I AM DONE

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // 将传入的队列用原子化包装，将包装的结果克隆两次
    // 1.Arc<T>是共享只读
    // Arc<T>是没有实现copy的，只有clone。copy会转移所有权，而clone只是引用或浅拷贝
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);// Arc::clone()需要传入一个Arc<T>类型数据的引用值
    let qc2 = Arc::clone(&qc);

    // 3.将tx用Arc包装，之后clone原本的Arc<tx>，再分别传入闭包。
    // 闭包获取所有权并消费了clone的Arc，没有数据在线程之间共享，因此可以通过编译
    let tx0 = Arc::new(tx);
    let tx1 = Arc::clone(&tx0);
    let tx2 = Arc::clone(&tx0);

    // 将队列的前半部分解引用，然后通过tx发送（注意：此时通过闭包转移了所有权）
    // 2.须知:没有实现copy的数据类型不能再线程间共享，而tx就没有实现copy
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 同理，但是发送的是后半部分
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建通道和队列，同时获取队列长度
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;


    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 可以发现，打印结果也不是顺序的
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
