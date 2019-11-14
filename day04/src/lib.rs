use counter::Counter;
use regex::Regex;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

const CHAR_A: u32 = 'a' as u32;
const ALPHA_LEN: u32 = 'z' as u32 - CHAR_A + 1;

pub struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let char_counts = self
            .name
            .chars()
            .filter(|&ch| ch != '-')
            .collect::<Counter<_>>();
        let mut top_chars = char_counts.iter().collect::<Vec<_>>();
        top_chars.sort_unstable_by(|(ch_a, count_a), (ch_b, count_b)| {
            count_b.cmp(count_a).then(ch_a.cmp(ch_b))
        });
        self.checksum
            == top_chars
                .iter()
                .take(5)
                .map(|&(ch, _)| *ch)
                .collect::<String>()
    }

    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|ch| {
                if ch.is_ascii_lowercase() {
                    ((CHAR_A + (ch as u32 - CHAR_A + self.id) % ALPHA_LEN)
                        as u8) as char
                } else if ch == '-' {
                    ' '
                } else {
                    ch
                }
            })
            .collect()
    }
}

pub fn part1(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.id)
        .sum()
}

pub fn part2(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .find(|room| room.decrypt() == "northpole object storage")
        .map(|room| room.id)
        .unwrap_or(0)
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new(r"^([[:alpha:]-]+)-([0-9]+)\[([[:alpha:]]+)\]").unwrap();

        let groups = pattern.captures(s).ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Invalid format")
        })?;

        let name = groups.get(1).unwrap().as_str().to_string();
        let id = groups
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let checksum = groups.get(3).unwrap().as_str().to_string();

        Ok(Room { name, id, checksum })
    }
}
