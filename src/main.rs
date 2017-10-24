#![feature(iterator_step_by)] 

extern crate easy_hash;

use easy_hash::{Sha256, Hasher, HashResult};
use std::thread;
use std::sync::mpsc;

const BASE: usize = 64;
const THREADS: usize = 4;
static DIFFICULTY: &'static str = "000000";

struct Solution(usize, String);

fn search(start_at: usize, sender: mpsc::Sender<Solution>) {
    for i in (start_at..).step_by(THREADS) {
        let hash: String = Sha256::hash(format!("{}", i * BASE).as_bytes()).hex();

        if hash.ends_with(DIFFICULTY) {
            sender.send(Solution(i, hash)).unwrap();
            break;
        }
    }
}

fn main() {
    println!("Attempting to find a number, which - while multiplied by {} and hashed using SHA-256 - will result in a hash ending with {}.", BASE, DIFFICULTY);

    /*
     * Here, we have 4 threads (as specified by the value of THREADS constant).
     * Thread 1 will start at number 1 and check 5, 9,  13 and so on.
     * Thread 2 will start at number 2 and check 6, 10, 14 and so on.
     * Thread 3 will start at number 3 and check 7, 11, 15 and so on.
     * Thread 4 will start at number 4 and check 8, 12, 16 and so on.
     * 
     * This way, we have 4 parallel threads of execution and we're sure
     * that each number will be examined exactly once.
     */
    let (sender, receiver) = mpsc::channel();
    for i in 1..THREADS+1 {
        let sender_n = sender.clone();
        thread::spawn(move || {
            search(i, sender_n);
        });
    }

    let Solution(i, hash) = receiver.recv().unwrap();
    println!("Found the solution.");
    println!("The number is: {}.", i);
    println!("Result hash: {}.", hash);
}
