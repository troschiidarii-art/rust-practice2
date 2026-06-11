fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    let mut freq = vec![0; 101];

    for &color in ar {
        freq[color as usize] += 1;
    }

    let mut pairs = 0;

    for count in freq {
        pairs += count / 2;
    }

    pairs
}