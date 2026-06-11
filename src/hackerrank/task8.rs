pub fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut freq = vec![0; 6]; // типы птиц 1..5

    for &x in &arr {
        freq[x as usize] += 1;
    }

    let mut best_type = 1;
    let mut best_count = 0;

    for t in 1..=5 {
        if freq[t] > best_count {
            best_count = freq[t];
            best_type = t as i32;
        }
    }

    best_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(arr), 4);
    }
}