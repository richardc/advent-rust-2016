#[aoc(day5, part1)]
fn solve(input: &str) -> String {
    let mut index = 0;
    let mut answer = String::new();
    for _ in 0..8 {
        loop {
            let digest = md5::compute(format!("{input}{index}").as_bytes());
            let hex = format!("{:x}", digest);
            index += 1;

            if &hex[0..5] == "00000" {
                answer.push(hex.chars().nth(5).unwrap());
                break;
            }
        }
    }
    answer
}

#[test]
fn test_password() {
    assert_eq!(&solve("abc"), "18f47a30");
}
