use itertools::Itertools;

#[derive(Debug)]
struct Room {
    name: String,
    sector: u32,
    checksum: String,
}

impl From<&str> for Room {
    fn from(input: &str) -> Self {
        let Some((name, rest)) = input.rsplit_once('-') else { unreachable!() };
        let Some((sector, checksum)) = rest.split_once('[') else { unreachable!() };

        Self {
            name: name.to_string(),
            sector: sector.parse().unwrap(),
            checksum: checksum[0..5].to_string(),
        }
    }
}

impl Room {
    fn checksum(&self) -> String {
        self.name
            .chars()
            .filter(|&c| c != '-')
            .counts()
            .iter()
            .sorted_by(|(ak, av), (bk, bv)| match Ord::cmp(bv, av) {
                std::cmp::Ordering::Equal => Ord::cmp(ak, bk),
                o => o,
            })
            .take(5)
            .map(|(k, _)| k)
            .collect()
    }

    fn legal(&self) -> bool {
        self.checksum == self.checksum()
    }
}

#[test]
fn test_room_checksum() {
    assert_eq!(
        &Room::from("aaaaa-bbb-z-y-x-123[abxyz]").checksum(),
        "abxyz"
    )
}

impl Room {
    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => unsafe {
                    char::from_u32_unchecked(
                        ((c as u32 - b'a' as u32 + self.sector) % 26) + b'a' as u32,
                    )
                },
            })
            .collect()
    }
}

#[test]
fn test_room_decrypt() {
    assert_eq!(
        &Room::from("qzmt-zixmtkozy-ivhz-343[aaaaa]").decrypt(),
        "very encrypted name"
    )
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<Room> {
    input.lines().map(Room::from).collect()
}

#[aoc(day4, part1)]
fn solve(rooms: &[Room]) -> u32 {
    rooms.iter().filter(|r| r.legal()).map(|r| r.sector).sum()
}

#[aoc(day4, part2)]
fn solve2(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|r| r.legal() && r.decrypt() == "northpole object storage")
        .map(|r| r.sector)
        .sum()
}
