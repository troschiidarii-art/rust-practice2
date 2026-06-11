pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;

    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    for x in max_a..=min_b {
        if a.iter().all(|n| x % n == 0)
            && b.iter().all(|n| n % x == 0)
        {
            count += 1;
        }
    }

    count
}

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