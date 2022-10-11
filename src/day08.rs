use itertools::{iproduct, Itertools};
use ndarray::prelude::*;

struct Display {
    data: Array2<bool>,
}

impl std::fmt::Debug for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .rows()
                .into_iter()
                .map(|r| String::from_iter(r.into_iter().map(|&c| if c { '#' } else { '.' })))
                .join("\n")
        )
    }
}

impl PartialEq<&str> for Display {
    fn eq(&self, other: &&str) -> bool {
        format!("{:?}", self) == *other.trim()
    }
}

impl Display {
    fn new(w: usize, h: usize) -> Self {
        Self {
            data: Array2::default((h, w)),
        }
    }

    fn lit(&self) -> usize {
        self.data.iter().filter(|&c| *c).count()
    }

    fn apply(&mut self, c: &str) {
        let toks = c.split_ascii_whitespace().collect_vec();
        if toks[0] == "rect" {
            let (w, h) = toks[1].split_once('x').unwrap();
            for (x, y) in iproduct!(0..w.parse().unwrap(), 0..h.parse().unwrap()) {
                self.data[[y, x]] = true;
            }
            return;
        }

        let (_, index) = toks[2].split_once('=').unwrap();
        let index = index.parse().unwrap();
        let by: usize = toks[4].parse().unwrap();
        let source = match toks[1] {
            "column" => self.data.column_mut(index),
            "row" => self.data.row_mut(index),
            _ => unreachable!(),
        };
        let mut rotated: Array1<bool> = Array::default((source.len(),));
        for i in 0..source.len() {
            rotated[(i + by) % source.len()] = source[i];
        }
        rotated.assign_to(source);
    }
}

#[test]
fn test_pixels() {
    let mut display = Display::new(7, 3);
    assert_eq!(display.lit(), 0);

    display.apply("rect 3x2");
    let example = r#"
###....
###....
.......
    "#;

    assert_eq!(display, example.trim());
    assert_eq!(display.lit(), 6);

    display.apply("rotate column x=1 by 1");
    let example = r#"
#.#....
###....
.#.....
    "#;

    assert_eq!(display, example.trim());
}

#[aoc(day8, part1)]
fn solve(input: &str) -> usize {
    let mut display = Display::new(50, 6);
    for c in input.lines() {
        display.apply(c);
    }
    display.lit()
}
