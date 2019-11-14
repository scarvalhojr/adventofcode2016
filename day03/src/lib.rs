fn is_triangle(x: u32, y: u32, z: u32) -> bool {
    x + y > z && x + z > y && y + z > x
}

pub fn part1(numbers: &[Vec<u32>]) -> usize {
    numbers
        .iter()
        .filter(|row| row.len() == 3 && is_triangle(row[0], row[1], row[2]))
        .count()
}

pub fn part2(numbers: &[Vec<u32>]) -> usize {
    (0..3)
        .map(|col| {
            numbers
                .chunks_exact(3)
                .filter_map(|chunk| {
                    let x = chunk[0].get(col)?;
                    let y = chunk[1].get(col)?;
                    let z = chunk[2].get(col)?;
                    if is_triangle(*x, *y, *z) {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum()
}
