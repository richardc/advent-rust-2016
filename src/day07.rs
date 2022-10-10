#[aoc(day7, part1)]
fn solve(input: &str) -> usize {
    input.lines().filter(|l| has_tls(l)).count()
}

fn has_tls(addr: &str) -> bool {
    let mut a = addr;
    let mut other = vec![];
    while let Some((left, rem)) = a.split_once('[') {
        other.push(left);
        let Some((mid, rest)) = rem.split_once(']') else { unreachable!() };
        if has_abba(mid) {
            return false;
        }
        a = rest
    }
    other.push(a);
    other.into_iter().any(has_abba)
}

fn has_abba(s: &str) -> bool {
    let b = s.as_bytes();
    for i in 0..b.len() - 3 {
        if b[i] == b[i + 3] && b[i + 1] == b[i + 2] && b[i] != b[i + 1] {
            return true;
        }
    }
    false
}

#[test]
fn test_has_tls() {
    assert!(has_tls("abba[mnop]qrst"));
    assert!(!has_tls("abcd[bddb]xyyx"));
    assert!(!has_tls("aaaa[qwer]tyui"));
    assert!(has_tls("ioxxoj[asdfgh]zxcvbn"));
    assert!(!has_tls("abba[mnop]qrst[abba]foos"));
}
