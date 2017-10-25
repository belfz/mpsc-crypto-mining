#![feature(iterator_step_by)]

extern crate easy_hash;

use easy_hash::{Sha256, Hasher, HashResult};
use std::thread;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

const BASE: usize = 42;
const THREADS: usize = 4;
static DIFFICULTY: &'static str = "000000";

struct Solution(usize, String);

fn search_for_solution(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    let mut iteration_no = 0;
    for i in (start_at..).step_by(THREADS) {
        let hash: String = Sha256::hash((i * BASE).to_string().as_bytes()).hex();
        
        if hash.ends_with(DIFFICULTY) {
            is_solution_found.store(true, Ordering::Relaxed);
            match sender.send(Solution(i, hash)) {
                Ok(_)  => {},
                Err(_) => println!("Receiver has stopped listening, dropping worker number {}.", start_at),
            }
            break;
        }
         
        if iteration_no % 1000 == 0 && is_solution_found.load(Ordering::Relaxed) {
            break;
        }
        iteration_no += 1;
    }
}

fn main() {
    println!("Attempting to find a number, which - while multiplied by {} and hashed using SHA-256 - will result in a hash ending with {}.", BASE, DIFFICULTY);
    println!("Please wait...");

    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();
    
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
    for i in 1..THREADS+1 {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            search_for_solution(i, sender_n, is_solution_found);
        });
    }

    /*
     * In a loop, receive messages from the producers.
     * As long as there is no solution found (None is received),
     * spin the loop. When the message with a solution arrives
     * (Some(Solution(i, hash))), print out the result and break
     * out from the loop.
     * 
     * Note that breaking out from the loop drops the receiver,
     * so that producers will receive an Err(_) on a next attempt to
     * send a message. This way, producers will "know" that the solution
     * was found by one of the worker threads and the receiver does no
     * longer "listen", so they can stop their work too.
     */
    loop {
        match receiver.recv() {
            Ok(Solution(i, hash)) => {
                println!("Found the solution.");
                println!("The number is: {}.", i);
                println!("Result hash: {}.", hash);
                break;
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}
