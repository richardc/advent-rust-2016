use itertools::Itertools;

fn expand(s: &str) -> String {
    let mirror = s.chars().rev().map(|c| if c == '0' { '1' } else { '0' });
    format!("{}0{}", s, String::from_iter(mirror))
}

#[cfg(test)]
#[test]
fn expand_1() {
    assert_eq!(expand("1"), "100");
}

#[cfg(test)]
#[test]
fn expand_0() {
    assert_eq!(expand("0"), "001");
}

#[cfg(test)]
#[test]
fn expand_11111() {
    assert_eq!(expand("11111"), "11111000000");
}

#[cfg(test)]
#[test]
fn expand_111100001010() {
    assert_eq!(expand("111100001010"), "1111000010100101011110000");
}

fn checksum(s: &str) -> String {
    let mut sum = String::from(s);
    loop {
        let bits = sum
            .chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' });
        sum = String::from_iter(bits);
        if sum.len() % 2 == 1 {
            return sum;
        }
    }
}

#[cfg(test)]
#[test]
fn checksum_110010110100() {
    assert_eq!(checksum("110010110100"), "100");
}

fn fill_disk(seed: &str, size: usize) -> String {
    let mut pattern = String::from(seed);
    while pattern.len() < size {
        pattern = expand(&pattern)
    }

    checksum(&pattern[..size])
}

#[cfg(test)]
#[test]
fn example_fill_disk() {
    assert_eq!(fill_disk("10000", 20), "01100");
}

#[aoc(day16, part1)]
fn solve(seed: &str) -> String {
    fill_disk(seed, 272)
}

#[aoc(day16, part2)]
fn solve2(seed: &str) -> String {
    fill_disk(seed, 35651584)
}
