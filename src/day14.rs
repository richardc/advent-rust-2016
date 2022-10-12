use itertools::Itertools;
use memoize::memoize;

fn three_in_a_row(hash: &[u8]) -> Option<u8> {
    for i in 0..hash.len() - 2 {
        if hash[i] == hash[i + 1] && hash[i] == hash[i + 2] {
            return Some(hash[i]);
        }
    }
    None
}

fn five_in_a_row(needle: u8, hash: &[u8]) -> bool {
    for i in 0..hash.len() - 5 {
        if needle == hash[i]
            && needle == hash[i + 1]
            && needle == hash[i + 2]
            && needle == hash[i + 3]
            && needle == hash[i + 4]
        {
            return true;
        }
    }
    false
}

#[memoize(Capacity: 1000)]
fn hasher(salt: String, index: usize) -> String {
    format!("{:x}", md5::compute(format!("{}{}", salt, index)))
}

#[aoc(day14, part1)]
fn solve(salt: &str) -> usize {
    let mut index = 0;
    for _digit in 0..64 {
        'checker: loop {
            let hash = hasher(salt.to_string(), index);
            index += 1;
            let threes = three_in_a_row(hash.as_bytes());
            if threes.is_none() {
                continue;
            }
            let three = threes.unwrap();

            for i in 0..1000 {
                let hash = hasher(salt.to_string(), index + i);
                if five_in_a_row(three, hash.as_bytes()) {
                    // Found one
                    //println!("Digit {} starting at {} found {}", _digit, index, index + i);
                    break 'checker;
                }
            }
        }
    }
    index - 1
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve("abc"), 22728)
}
