fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len() as i32;

    let mut left_diag = 0;
    let mut right_diag = 0;

    for i in 0..n {
        left_diag += arr[i as usize][i as usize];
        right_diag += arr[i as usize][(n - 1 - i) as usize];
    }

    (left_diag - right_diag).abs()
}