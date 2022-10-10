#[aoc(day7, part1)]
fn solve(input: &str) -> usize {
    input.lines().filter(|l| has_tls(l)).count()
}

fn parse_addr(addr: &str) -> (Vec<&str>, Vec<&str>) {
    let mut addrs = vec![];
    let mut nets = vec![];
    let mut a = addr;
    while let Some((left, rem)) = a.split_once('[') {
        addrs.push(left);
        let Some((mid, rest)) = rem.split_once(']') else { unreachable!() };
        nets.push(mid);
        a = rest
    }
    addrs.push(a);
    (addrs, nets)
}

fn has_tls(addr: &str) -> bool {
    let (addrs, nets) = parse_addr(addr);
    !nets.into_iter().any(has_abba) && addrs.into_iter().any(has_abba)
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
