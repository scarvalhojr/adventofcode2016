use counter::Counter;
use std::collections::BTreeMap;

pub fn count_chars(
    messages: &[String],
) -> BTreeMap<usize, Counter<char, usize>> {
    let mut counters = BTreeMap::new();
    for msg in messages {
        for (pos, ch) in msg.chars().enumerate() {
            let counter = counters.entry(pos).or_insert_with(Counter::new);
            counter
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    counters
}

pub fn part1(counters: &BTreeMap<usize, Counter<char, usize>>) -> String {
    counters
        .values()
        .filter_map(|counter| {
            counter.most_common().iter().map(|&(ch, _)| ch).nth(0)
        })
        .collect()
}

pub fn part2(counters: &BTreeMap<usize, Counter<char, usize>>) -> String {
    counters
        .values()
        .filter_map(|counter| {
            counter.most_common().iter().map(|&(ch, _)| ch).last()
        })
        .collect()
}
