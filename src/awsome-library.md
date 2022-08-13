# Awesome library

## 错误处理


> 2022-08-13 Sat

- [anyhow](https://github.com/dtolnay/anyhow)
  - Flexible concrete Error type built on `std::error::Error`
  - 基于 `std::error::Error` 构建的灵活的具体错误类型

```rust
use anyhow::{ensure, format_err, Ok, Result};
```

嵌套路径引入模块，清理众多 use 语句
