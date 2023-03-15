# Tests

## lib.rs

Replace the `add()` and test method in the root `lib.rs` file to include a test of the `sample::point()` function:

```rust
pub mod sample;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let result = sample::point(-0.5, 0.0, 1000);
        assert_eq!(result, 1000);
    }
}
```

## Try it out

Run the tests and verify that they pass:

```bash
cargo test
```
