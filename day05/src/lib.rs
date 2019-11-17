use md5::Context;
use std::char::from_digit;
use std::collections::BTreeMap;

const PWD_LEN: usize = 8;

pub fn part1(door_id: &str) -> String {
    (0..)
        .map(|seq| {
            let mut hash = Context::new();
            hash.consume(door_id.as_bytes());
            hash.consume(seq.to_string().as_bytes());
            hash.compute()
        })
        .filter(|hash| hash[0] | hash[1] | (0xF0 & hash[2]) == 0)
        .filter_map(|hash| from_digit((0x0F & hash[2]) as u32, 16))
        .take(PWD_LEN)
        .collect()
}

pub fn part2(door_id: &str) -> String {
    let mut password = BTreeMap::new();
    for (pos, ch) in (0..)
        .map(|seq| {
            let mut hash = Context::new();
            hash.consume(door_id.as_bytes());
            hash.consume(seq.to_string().as_bytes());
            hash.compute()
        })
        .filter(|hash| hash[0] | hash[1] | (0xF0 & hash[2]) == 0)
        .map(|hash| {
            (
                0x0F & hash[2],
                from_digit((hash[3] >> 4) as u32, 16).unwrap(),
            )
        })
        .filter(|&(pos, _)| (pos as usize) < PWD_LEN)
    {
        password.entry(pos).or_insert(ch);
        if password.len() == PWD_LEN {
            break;
        }
    }
    password.values().collect()
}
