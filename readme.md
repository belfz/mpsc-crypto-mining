# mpsc-crypto-mining

An example of using `std::sync::mpsc` with multiple threads of execution.
The computation-heavy algorithm used in this example is a **basic implementation of an idea** behind cryptocurrencies mining process (so-called "Proof of Work", or solving the cryptographic problem).

### About the cryptographic algorithm
An example: given a number 42 (the "constant base"), find a number _x_ to multiply it by, so that the result of this multiplication hashed by SHA-256 will produce a hash string ending with `000000`.

The actual solution is 3,305,951, which - while multiplied by 42 gives 138,849,942, producing a `2a44903ffc6affe69d514ffe47721cc3a6475cbb43b37538686f2c5b46000000` hash.

### About `std::sync::mpsc`
This example uses `std::sync::mpsc` as a channel of communication between four worker threads and the main thread. Each worker thread analyses unique numbers (by starting the iterations at different "points" and having constant step). For more info, see the large comment inside `src/main.rs`'s `main()` function.

### How-to

**For maximum performance, instead of running with bare `cargo run` I recommend to add the `--relase` flag:**
1. `cd` to project's root,
2. run `cargo run --release` and enjoy!

Feel free to experiment - change the value of `DIFFICULTY` constant in `src/main.rs` and/or `THREADS`! Please note that a change by one extra desired character in `DIFFICULTY` (eg. one more `0`) can increase the overall difficulty of the problem dramatically, thus noticeably extending the time needed to find the solution!

### License
MIT
