#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Information message");
    debug!("Debugging message");
}

// joe@MX:~/i/rust-notes/cpfrp/ch01/e6-use-env-logger$ cargo build
//    Compiling e6-use-env-logger v0.1.0 (/home/joe/i/rust-notes/cpfrp/ch01/e6-use-env-logger)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.87s
// joe@MX:~/i/rust-notes/cpfrp/ch01/e6-use-env-logger$ RUST_LOG=debug ./target/debug/e6-use-env-logger
// [2022-12-15T08:34:27Z ERROR e6_use_env_logger] Error message
// [2022-12-15T08:34:27Z WARN  e6_use_env_logger] Warning message
// [2022-12-15T08:34:27Z INFO  e6_use_env_logger] Information message
// [2022-12-15T08:34:27Z DEBUG e6_use_env_logger] Debugging message
// joe@MX:~/i/rust-notes/cpfrp/ch01/e6-use-env-logger$ RUST_LOG=info ./target/debug/e6-use-env-logger
// [2022-12-15T08:34:54Z ERROR e6_use_env_logger] Error message
// [2022-12-15T08:34:54Z WARN  e6_use_env_logger] Warning message
// [2022-12-15T08:34:54Z INFO  e6_use_env_logger] Information message
// joe@MX:~/i/rust-notes/cpfrp/ch01/e6-use-env-logger$ RUST_LOG=warn ./target/debug/e6-use-env-logger
// [2022-12-15T08:34:59Z ERROR e6_use_env_logger] Error message
// [2022-12-15T08:34:59Z WARN  e6_use_env_logger] Warning message
// joe@MX:~/i/rust-notes/cpfrp/ch01/e6-use-env-logger$ RUST_LOG=error ./target/debug/e6-use-env-logger
// [2022-12-15T08:35:03Z ERROR e6_use_env_logger] Error message
