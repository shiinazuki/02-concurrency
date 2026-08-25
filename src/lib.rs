//! RUST CONCURRENCY
//!
//! 这里是**库**目标，业务逻辑写在这一侧；`src/main.rs` 只做参数解析、日志初始化
//! 和错误收口，然后调用这里的函数。
//!
//! 这么分不是为了好看：`main.rs` 里的东西集成测试（`tests/`）、benchmark 和
//! doctest 都够不着，逻辑留在那边就只能靠手工跑一遍程序来验证。

mod error;
mod matrix;
mod metrics;
mod vector;

pub use matrix::{Matrix, multiply};
pub use metrics::{AtomicMetrics, LockedMetrics, ShardedMetrics};
pub use vector::{Vector, dot_product};

pub use crate::error::{Error, Result};

/// 把两个数相加。
///
/// # Examples
///
/// ```
/// let sum = concurrency::add(1, 2);
/// assert_eq!(sum, 3);
/// ```
#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// 生成问候语。
///
/// # Errors
///
/// `name` 去掉首尾空白后为空时返回 [`Error::EmptyName`]。
///
/// # Examples
///
/// ```
/// # fn main() -> concurrency::Result<()> {
/// let msg = concurrency::greet("world")?;
/// assert_eq!(msg, "Hello, world!");
///
/// let blank = concurrency::greet("  ");
/// assert!(blank.is_err());
/// # Ok(())
/// # }
/// ```
pub fn greet(name: &str) -> Result<String> {
    if name.trim().is_empty() {
        return Err(Error::EmptyName);
    }
    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::{add, greet};

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn greet_formats_message() {
        assert_eq!(greet("world").unwrap(), "Hello, world!");
    }

    #[test]
    fn greet_rejects_blank_name() {
        assert!(greet("   ").is_err());
    }

    /// 异步测试用 `#[tokio::test]`，它会自动起一个 runtime，不需要手写 `block_on`。
    #[tokio::test]
    async fn async_test_works() {
        assert_eq!(add(1, 1), 2);
    }
}
