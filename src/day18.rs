use itertools::Itertools;

fn is_safe(t: (bool, bool, bool)) -> bool {
    #[allow(clippy::match_like_matches_macro)] // matches! version reads poorly
    match t {
        (false, false, true) => false,
        (true, false, false) => false,
        (true, true, false) => false,
        (false, true, true) => false,
        _ => true,
    }
}

fn next_line(curr: &[bool]) -> Vec<bool> {
    [&[true], curr, &[true]]
        .concat()
        .into_iter()
        .tuple_windows()
        .map(is_safe)
        .collect()
}

fn parse(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '.').collect()
}

#[allow(dead_code)] // used by tests and while poking
fn view(l: &[bool]) -> String {
    String::from_iter(l.iter().map(|&safe| if safe { '.' } else { '^' }))
}

#[cfg(test)]
mod next_line {
    use super::*;
    #[test]
    fn step1() {
        assert_eq!(view(&next_line(&parse("..^^."))), ".^^^^");
    }

    #[test]
    fn step2() {
        assert_eq!(view(&next_line(&parse(".^^^^"))), "^^..^");
    }
}

fn safe(input: &str, rows: usize) -> usize {
    let mut count = 0;
    let mut line = parse(input);
    // println!("{}", view(&line));
    count += line.iter().filter(|&c| *c).count();
    for _ in 0..rows - 1 {
        line = next_line(&line);
        // println!("{}", view(&line));
        count += line.iter().filter(|&c| *c).count();
    }

    count
}

#[cfg(test)]
mod safe {
    use super::*;

    #[test]
    fn five_by_three() {
        assert_eq!(safe("..^^.", 3), 6);
    }

    #[test]
    fn ten_by_ten() {
        assert_eq!(safe(".^^.^.^^^^", 10), 38);
    }
}

#[aoc(day18, part1)]
fn solve(input: &str) -> usize {
    safe(input, 40)
}

#[aoc(day18, part2)]
fn solve2(input: &str) -> usize {
    safe(input, 400000)
}
