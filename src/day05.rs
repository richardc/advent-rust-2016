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
fn test_solve() {
    assert_eq!(&solve("abc"), "18f47a30");
}

#[aoc(day5, part2)]
fn solve2(input: &str) -> String {
    let mut interesting = 0;
    let mut answer = ['_'; 8];
    while answer.iter().any(|&c| c == '_') {
        let digest = md5::compute(format!("{input}{interesting}").as_bytes());
        let hex = format!("{:x}", digest);
        interesting += 1;

        if &hex[0..5] == "00000" {
            let index = usize::from_str_radix(&hex[5..=5], 16).unwrap();
            if index >= answer.len() {
                continue;
            }
            if answer[index] == '_' {
                let digit = hex.chars().nth(6).unwrap();
                answer[index] = digit;
                println!("{} {}", interesting, String::from_iter(answer));
            }
        }
    }
    String::from_iter(answer)
}

#[test]
fn test_solve2() {
    assert_eq!(&solve2("abc"), "05ace8e3");
}
