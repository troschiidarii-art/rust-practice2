#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        assert_eq!(get_total_x(a, b), 3);
    }
}