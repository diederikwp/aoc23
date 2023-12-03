use advent_of_code::puzzles::d03::{GridSlot, Number, Schematic};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the schematic
    let schematic = Schematic::new(input);
    let grid = schematic.grid();
    let mut selected_nums = vec![false; schematic.numbers().len()];

    // Mark all numbers adjacent to a symbol in `selected_nums`
    for sym in schematic.symbols() {
        for (x, y) in [
            (sym.x - 1, sym.y - 1),
            (sym.x, sym.y - 1),
            (sym.x + 1, sym.y - 1),
            (sym.x - 1, sym.y),
            (sym.x + 1, sym.y),
            (sym.x - 1, sym.y + 1),
            (sym.x, sym.y + 1),
            (sym.x + 1, sym.y + 1),
        ] {
            if let Some(GridSlot::Number(i)) = grid.get([x, y]) {
                selected_nums[*i] = true;
            }
        }
    }

    // Sum the values of those numbers
    Some(
        schematic
            .numbers()
            .iter()
            .filter(|Number { idx: i, val: _ }| selected_nums[*i])
            .map(|Number { idx: _, val }| val)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
