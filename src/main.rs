extern crate easy_hash;

use easy_hash::{Sha256, Hasher, HashResult};

const BASE: u64 = 64;

fn search(lower_bound: u64, difficulty: &str) {
    for i in lower_bound.. {
        let hash: String = Sha256::hash(format!("{}", i * BASE).as_bytes()).hex();

        if hash.ends_with(difficulty) {
            println!("the solution is {}", i);
            println!("hash: {}", hash);
            break;
        }
    }
}

fn main() {
    search(1u64, "0000");
}
