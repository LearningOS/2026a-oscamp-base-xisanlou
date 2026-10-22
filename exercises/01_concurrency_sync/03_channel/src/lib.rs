//! # Channel Communication
//!
//! In this exercise, you will use `std::sync::mpsc` channels to pass messages between threads.
//!
//! ## Concepts
//! - `mpsc::channel()` creates a multiple producer, single consumer channel
//! - `Sender::send()` sends a message
//! - `Receiver::recv()` receives a message
//! - Multiple producers can be created via `Sender::clone()`

use std::sync::mpsc;
use std::thread;

/// Create a producer thread that sends each element from items into the channel.
/// The main thread receives all messages and returns them.
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel
    // TODO: Spawn thread to send each element in items
    // TODO: In main thread, receive all messages and collect into Vec
    // Hint: When all Senders are dropped, recv() returns Err
    // ****** START xisanlou add at 202603 No.1
    //todo!()
    let mut result: Vec<String> = vec![];
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for text in items {
            sender.send(text).unwrap();
        }
    });

    handle.join().unwrap();

    while let Ok(text) = receiver.recv() {
        result.push(text);
    }

    result
    // ****** END   xisanlou add at 202603 No.1
}

/// Create `n_producers` producer threads, each sending a message in format `"msg from {id}"`.
/// Collect all messages, sort them lexicographically, and return.
///
/// Hint: Use `tx.clone()` to create multiple senders. Note that the original tx must also be dropped.
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: Create channel
    // TODO: Clone a sender for each producer
    // TODO: Remember to drop the original sender, otherwise receiver won't finish
    // TODO: Collect all messages and sort
    // ****** START xisanlou add at 202603 No.2
    let mut result: Vec<String> = vec![];
    let (sender, receiver) =mpsc::channel();
    // channel自身有同步机制，所以可以不用join
    for i in 0..n_producers {
        let sender2 = sender.clone();
        
        thread::spawn(move || {
            sender2.send(format!("msg from {}", i).to_string()).unwrap();
        });   
        
    }
    //因为多clone了一个sender，索引需要drop主线程的sender
    drop(sender);
    while let Ok(text) = receiver.recv() {
        result.push(text);
    }
    // sort是原地修改，返回的不是排序后对象
    result.sort();
    result
    // ****** END   xisanlou add at 202603 No.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(result, vec![
            "msg from 0".to_string(),
            "msg from 1".to_string(),
            "msg from 2".to_string(),
        ]);
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
