use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use async_trait::async_trait;
use tokio::count;

fn thread_main() {
    println!("# Thread");
    let handle = thread::spawn(|| {
        println!("I'm thread");
    });
    dbg!(handle.join()).expect("TODO: panic message");

    let mut handles = Vec::new();
    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("I'm thread no.{}", x);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}

fn thread_with_shared_memory() {
    println!("# Thread with shared memory");
    // let mut handles = Vec::new();
    // let mut data = vec![1; 10];
    //
    // for x in 0..10 {
    //     handles.push(thread::spawn(move || {
    //         data[x] += 1; // value moved into closure here, in previous iteration of loop
    //     }));
    // }
    // for handle in handles {
    //     let _ = handle.join();
    // }
    // dbg!(data);

    // use std::rc::Rc
    // let mut handles = Vec::new();
    // let mut data = Rc::new(vec![1; 10]);
    //
    // for x in 0..10 { // cannot be sent between threads safely
    //     handles.push(thread::spawn(move || {
    //         data[x] += 1;
    //     }));
    // }
    // for handle in handles {
    //     let _ = handle.join();
    // }
    // dbg!(data);

    // use std::sync::Arc
    // let mut handles = Vec::new();
    // let mut data = Arc::new(vec![1; 10]);
    //
    // for x in 0..10 {
    //     handles.push(thread::spawn(move || {
    //         data[x] += 1; // cannot borrow as mutable
    //     }));
    // }
    // for handle in handles {
    //     let _ = handle.join();
    // }
    // dbg!(data);

    // use std::sync::{Arc, Mutex}
    let mut handles = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // use lock by mutex here
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}

fn message_passing() {
    println!("# Message Passing");
    // use std::sync::mpsc
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });
    let _ = tx.send("send message");
    let _ = handle.join();

    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut send_channels = Vec::new();
    let mut recv_channels = Vec::new();
    for _ in 0..10 {
        let (send_tx, send_rx) = mpsc::channel();
        let (recv_tx, recv_rx) = mpsc::channel();
        send_channels.push(send_tx);
        recv_channels.push(recv_rx);
        handles.push(thread::spawn(move || {
            let mut data = send_rx.recv().unwrap();
            data += 1;
            let _ = recv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = send_channels[x].send(data[x]);
    }
    for x in 0..10 {
        data[x] = recv_channels[x].recv().unwrap();
    }
    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

fn countdown() {
    println!("# Countdown");
    struct Countdown(u32);
    impl Future for Countdown {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 == 0 {
                Poll::Ready("zero".to_string())
            } else {
                println!("{}", self.0);
                self.0 -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let countdown_future_1 = Countdown(10);
    let countdown_future_2 = Countdown(20);
    let countdown_set = join_all(vec![countdown_future_1, countdown_future_2]);
    let res = executor::block_on(countdown_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}

async fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[tokio::main]
// #[async_std::main]
pub async fn main() {
    thread_main();
    thread_with_shared_memory();
    message_passing();
    countdown();
    println!("# Async");
    let ans = add(2 ,3).await;
    println!("{}", ans);
}

// trait AsyncTrait {
//     async fn f() {
//         println!("Couldn't compile");
//     }
// }
#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("Could compile");
    }
}
