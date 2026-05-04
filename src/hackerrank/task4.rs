pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .map(|x| a + x)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .map(|x| b + x)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}