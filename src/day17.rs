use pathfinding::prelude::bfs;
use pathfinding::prelude::bfs_reach;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Maze {
    salt: String,
    path: Vec<char>,
    x: u8,
    y: u8,
}

fn door_open(c: u8) -> bool {
    (b'b'..=b'f').contains(&c)
}

impl Maze {
    fn new(salt: &str) -> Self {
        Self {
            salt: salt.to_string(),
            path: vec![],
            x: 1,
            y: 1,
        }
    }

    fn make_move(&self, step: char) -> Self {
        let mut moved = self.clone();
        moved.path.push(step);
        match step {
            'U' => moved.y -= 1,
            'D' => moved.y += 1,
            'L' => moved.x -= 1,
            'R' => moved.x += 1,
            _ => (),
        }
        //println!("{},{} {}", moved.x, moved.y, String::from_iter(&moved.path));
        moved
    }

    fn successors(&self) -> Vec<Self> {
        if self.success() {
            return vec![];
        }

        let hash = format!(
            "{:x}",
            md5::compute(format!("{}{}", self.salt, String::from_iter(&self.path)))
        );
        let bytes = hash.as_bytes();
        let (up, down, left, right) = (bytes[0], bytes[1], bytes[2], bytes[3]);

        let mut succ = vec![];
        if self.y > 1 && door_open(up) {
            succ.push(self.make_move('U'))
        }
        if self.y < 4 && door_open(down) {
            succ.push(self.make_move('D'))
        }
        if self.x > 1 && door_open(left) {
            succ.push(self.make_move('L'))
        }
        if self.x < 4 && door_open(right) {
            succ.push(self.make_move('R'))
        }

        succ
    }

    fn success(&self) -> bool {
        (self.x, self.y) == (4, 4)
    }
}

#[cfg(test)]
mod maze_successors {
    use super::*;
    #[test]
    fn start() {
        let start = Maze::new("hijkl");
        assert_eq!(start.successors(), vec![start.make_move('D')])
    }

    #[test]
    fn d() {
        let start = Maze::new("hijkl").make_move('D');
        assert_eq!(
            start.successors(),
            vec![start.make_move('U'), start.make_move('R')]
        )
    }

    #[test]
    fn du() {
        let start = Maze::new("hijkl").make_move('D').make_move('U');
        assert_eq!(start.successors(), vec![start.make_move('R')])
    }

    #[test]
    fn dr() {
        let start = Maze::new("hijkl").make_move('D').make_move('R');
        assert_eq!(start.successors(), vec![])
    }

    #[test]
    fn dur() {
        let start = Maze::new("hijkl")
            .make_move('D')
            .make_move('U')
            .make_move('R');
        assert_eq!(start.successors(), vec![])
    }
}

#[aoc(day17, part1)]
fn shortest(salt: &str) -> String {
    let maze = Maze::new(salt);
    if let Some(path) = bfs(&maze, |m| m.successors(), |m| m.success()) {
        //println!("Found: {:?} {}", path, _cost);
        return String::from_iter(&path.last().unwrap().path);
    }

    String::from("Not found")
}

#[cfg(test)]
#[test]
fn shortest_ihgpwlah() {
    assert_eq!(shortest("ihgpwlah"), "DDRRRD")
}

#[cfg(test)]
#[test]
fn shortest_kglvqrro() {
    assert_eq!(shortest("kglvqrro"), "DDUDRLRRUDRD")
}

#[cfg(test)]
#[test]
fn shortest_ulqzkmiv() {
    assert_eq!(shortest("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR")
}

#[aoc(day17, part2)]
fn longest(salt: &str) -> usize {
    let maze = Maze::new(salt);
    if let Some(path) = bfs_reach(maze, |m| m.successors())
        .filter(|m| m.success())
        .last()
    {
        return path.path.len();
    }

    0
}

#[cfg(test)]
#[test]
fn longest_ihgpwlah() {
    assert_eq!(longest("ihgpwlah"), 370)
}

#[cfg(test)]
#[test]
fn longest_kglvqrro() {
    assert_eq!(longest("kglvqrro"), 492)
}

#[cfg(test)]
#[test]
fn longest_ulqzkmiv() {
    assert_eq!(longest("ulqzkmiv"), 830)
}
