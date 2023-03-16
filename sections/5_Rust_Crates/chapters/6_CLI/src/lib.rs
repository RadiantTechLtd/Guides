pub mod colour;
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
