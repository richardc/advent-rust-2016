#[aoc(day9, part1)]
fn solve(mut input: &[u8]) -> usize {
    let mut count = 0;
    while !input.is_empty() {
        if input[0] == b'(' {
            let end = input.iter().position(|&c| c == b')').unwrap();
            let (size, repeated) = std::str::from_utf8(&input[1..end])
                .unwrap()
                .split_once('x')
                .unwrap();
            let size: usize = size.parse().unwrap();
            let repeated: usize = repeated.parse().unwrap();

            input = &input[end + size + 1..];
            count += size * repeated;
        } else {
            input = &input[1..];
            count += 1;
        }
    }
    count
}

#[test]
fn test_solve() {
    assert_eq!(solve(b"ADVENT"), 6);
    assert_eq!(solve(b"A(1x5)BC"), 7);
    assert_eq!(solve(b"(3x3)XYZ"), 9);
    assert_eq!(solve(b"A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(solve(b"(6x1)(1x3)A"), 6);
    assert_eq!(solve(b"X(8x2)(3x3)ABCY"), 18);
}
