fn main() {
    let mut _a = 7;
    let _ref_to_a = &_a;
    _a = 9;
}

// joe@MX:~/i/rust-notes/cpfrp/ch01/e1-non-lexical$ cargo run
//    Compiling e1-non-lexical v0.1.0 (/home/joe/i/rust-notes/cpfrp/ch01/e1-non-lexical)
// error[E0506]: cannot assign to `_a` because it is borrowed
//  --> src/main.rs:4:5
//   |
// 3 |     let _ref_to_a = &_a;
//   |                      -- borrow of `_a` occurs here
// 4 |     _a = 9;
//   |     ^^^^^^ assignment to borrowed `_a` occurs here

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0506`.
// error: Could not compile `e1-non-lexical`.

// To learn more, run the command again with --verbose.
