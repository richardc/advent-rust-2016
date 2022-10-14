#[derive(Debug)]
struct Span {
    start: u32,
    end: u32,
}

impl Span {
    fn new(start: u32, end: u32) -> Self {
        let (start, end) = (std::cmp::min(start, end), std::cmp::max(start, end));
        Self { start, end }
    }
}

#[derive(Default, Debug)]
struct Firewall {
    blacklist: Vec<Span>,
}

#[aoc_generator(day20)]
fn generate(input: &str) -> Firewall {
    let mut fw = Firewall::default();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();

        fw.blacklist.push(Span::new(from, to));
        fw.blacklist.sort_by(|a, b| Ord::cmp(&a.start, &b.start));
    }
    fw
}

impl Firewall {
    fn first_free(&self) -> u32 {
        if self.blacklist[0].start > 0 {
            return 0;
        }

        let mut span = &self.blacklist[0];
        for next in &self.blacklist[1..] {
            if span.end + 1 < next.start {
                return span.end + 1;
            }
            // We didn't weed out overlapping spans
            if span.end > next.end {
                continue;
            }
            span = next
        }
        span.end + 1
    }

    fn all_free(&self, max: u32) -> u32 {
        let mut count = 0;
        let mut span = &self.blacklist[0];

        for next in &self.blacklist[1..] {
            if span.end < next.start {
                count += next.start - span.end - 1;
            }
            if next.end > span.end {
                span = next;
                continue;
            }
        }
        count += max - span.end;
        count
    }
}

#[test]
fn test_firewall_firstfree() {
    assert_eq!(generate(include_str!("day20_example.txt")).first_free(), 3)
}

#[test]
fn test_firewall_allfree() {
    assert_eq!(generate(include_str!("day20_example.txt")).all_free(9), 2)
}

#[aoc(day20, part1)]
fn solve(fw: &Firewall) -> u32 {
    fw.first_free()
}

#[aoc(day20, part2)]
fn solve2(fw: &Firewall) -> u32 {
    fw.all_free(u32::MAX)
}
