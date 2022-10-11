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

#[aoc(day9, part2)]
fn solve2(mut input: &[u8]) -> usize {
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

            count += repeated * solve2(&input[end + 1..end + size + 1]);
            input = &input[end + size + 1..];
        } else {
            count += 1;
            input = &input[1..];
        }
    }
    count
}

#[test]
fn test_solve2() {
    assert_eq!(solve2(b"(3x3)XYZ"), 9);
    assert_eq!(solve2(b"X(8x2)(3x3)ABCY"), 20);
    assert_eq!(solve2(b"(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(
        solve2(b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );
}
