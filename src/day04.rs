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

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<Room> {
    input.lines().map(Room::from).collect()
}

#[aoc(day4, part1)]
fn solve(rooms: &[Room]) -> u32 {
    rooms.iter().filter(|r| r.legal()).map(|r| r.sector).sum()
}
