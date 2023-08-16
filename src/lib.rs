pub fn hello() {
    println!("Hello, from lib.rs!");
}

/// Adds two integers together.
///
/// # Examples
///
/// ```
/// let result = rust_simple_template::plus(5, 10);
/// assert_eq!(result, 15);
/// ```
pub fn plus(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        assert_eq!(plus(5, 10), 15);
    }
}
