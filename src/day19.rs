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
