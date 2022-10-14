#[aoc_generator(day19)]
fn generate(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day19, part1, slice)]
fn josephus(input: &usize) -> usize {
    // First find: 2^m + l for the highest power of 2
    // Answer: is 2 * l + 1.

    // Numberphile's video shows a neat trick with the binary representation,
    // first make the value binary, then stringwise rotate it, such that MSB
    // becomes the LSB.
    let bits = format!("{:b}", *input);
    let rotated = format!("{}1", &bits[1..]);
    usize::from_str_radix(&rotated, 2).unwrap()
}

#[aoc(day19, part1, rotate)]
fn josephus_fiddling(input: &usize) -> usize {
    let mut bits = format!("{:b}", *input);
    unsafe { bits.as_bytes_mut() }.rotate_left(1);
    usize::from_str_radix(&bits, 2).unwrap()
}

#[aoc(day19, part1, bitmath)]
fn josephus_bitmath(input: &usize) -> usize {
    (*input ^ (1 << (usize::BITS - input.leading_zeros() - 1))) * 2 + 1
}

#[cfg(test)]
#[test]
fn test_josephus() {
    assert_eq!(josephus(&5), 3);
}

#[aoc(day19, part2)]
fn joe2(players: &usize) -> usize {
    // 0 and 1 player games are basecases
    if *players < 2 {
        return *players;
    }
    // Analysis shows W = P - 3^m
    let mut power_of_three = 1;
    while power_of_three * 3 < *players {
        power_of_three *= 3
    }
    *players - power_of_three
}

// This idiom to emulate table-based tests is growing on me a bit - as distinct
// functions they run as seperate threads, plus with some sympathetic naming you
// can see the case at a glance.
#[cfg(test)]
mod joe2 {
    use super::*;
    #[test]
    fn t_1() {
        assert_eq!(joe2(&1), 1);
    }
    #[test]
    fn t_2() {
        assert_eq!(joe2(&2), 1);
    }
    #[test]
    fn t_3() {
        assert_eq!(joe2(&3), 2);
    }
    #[test]
    fn t_4() {
        assert_eq!(joe2(&4), 1);
    }
    #[test]
    fn t_5() {
        assert_eq!(joe2(&5), 2);
    }
    #[test]
    fn t_6() {
        assert_eq!(joe2(&6), 3);
    }
    #[test]
    fn t_7() {
        assert_eq!(joe2(&7), 4);
    }
    #[test]
    fn t_8() {
        assert_eq!(joe2(&8), 5);
    }
    #[test]
    fn t_9() {
        assert_eq!(joe2(&9), 6);
    }
    #[test]
    fn t_10() {
        assert_eq!(joe2(&10), 1);
    }
}
