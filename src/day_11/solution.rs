use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    row_len: u8,
    seats: Vec<SeatRow>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            row_len: 0,
            seats: Vec::new(),
        }
    }

    fn adjacent(&self, row: usize, col: u8) -> usize {
        let mut adjacent = 0;
        if row > 0 {
            let seat_row = &self.seats[row - 1];

            if col > 0 && seat_row.is_occupied(col - 1) {
                adjacent += 1;
            }

            if seat_row.is_occupied(col) {
                adjacent += 1;
            }

            if col < self.row_len - 1 && seat_row.is_occupied(col + 1) {
                adjacent += 1;
            }
        }

        let seat_row = &self.seats[row];

        if col > 0 && seat_row.is_occupied(col - 1) {
            adjacent += 1;
        }

        if col < self.row_len - 1 && seat_row.is_occupied(col + 1) {
            adjacent += 1;
        }

        if row < self.seats.len() - 1 {
            let seat_row = &self.seats[row + 1];

            if col > 0 && seat_row.is_occupied(col - 1) {
                adjacent += 1;
            }

            if seat_row.is_occupied(col) {
                adjacent += 1;
            }

            if col < self.row_len - 1 && seat_row.is_occupied(col + 1) {
                adjacent += 1;
            }
        }
        adjacent
    }

    fn visibly_adjacent(&self, row: usize, col: usize) -> usize {
        let mut adjacent = 0;
        if row > 0 {
            // TL
            for diff in 1..=row {
                if diff > col {
                    break;
                }
                let col_idx = (col - diff) as u8;
                let row_idx = row - diff;

                if !self.seats[row_idx].is_floor(col_idx) {
                    if self.seats[row_idx].is_occupied(col_idx) {
                        adjacent += 1;
                    }
                    break;
                }
            }

            // TC
            for diff in 1..=row {
                let row_idx = row - diff;

                if !self.seats[row_idx].is_floor(col as u8) {
                    if self.seats[row_idx].is_occupied(col as u8) {
                        adjacent += 1;
                    }
                    break;
                }
            }

            // TR
            for diff in 1..=row {
                if self.row_len as usize - diff <= col {
                    break;
                }
                let col_idx = (col + diff) as u8;
                let row_idx = row - diff;

                if !self.seats[row_idx].is_floor(col_idx) {
                    if self.seats[row_idx].is_occupied(col_idx) {
                        adjacent += 1;
                    }
                    break;
                }
            }
        }

        // ML
        for diff in 1..=col {
            let col_idx = (col - diff) as u8;

            if !self.seats[row].is_floor(col_idx) {
                if self.seats[row].is_occupied(col_idx) {
                    adjacent += 1;
                }
                break;
            }
        }

        // MR
        for diff in 1..=(self.row_len as usize - col) {
            let col_idx = (col + diff) as u8;

            if !self.seats[row].is_floor(col_idx) {
                if self.seats[row].is_occupied(col_idx) {
                    adjacent += 1;
                }
                break;
            }
        }

        // BL
        for diff in 1..(self.seats.len() - row) {
            if diff > col {
                break;
            }
            let col_idx = (col - diff) as u8;
            let row_idx = row + diff;

            if !self.seats[row_idx].is_floor(col_idx) {
                if self.seats[row_idx].is_occupied(col_idx) {
                    adjacent += 1;
                }
                break;
            }
        }

        // BM
        for diff in 1..(self.seats.len() - row) {
            let row_idx = row + diff;

            if !self.seats[row_idx].is_floor(col as u8) {
                if self.seats[row_idx].is_occupied(col as u8) {
                    adjacent += 1;
                }
                break;
            }
        }

        // BR
        for diff in 1..(self.seats.len() - row) {
            if self.row_len as usize - diff <= col {
                break;
            }
            let col_idx = (col + diff) as u8;
            let row_idx = row + diff;

            if !self.seats[row_idx].is_floor(col_idx) {
                if self.seats[row_idx].is_occupied(col_idx) {
                    adjacent += 1;
                }
                break;
            }
        }

        adjacent
    }
}

// u128 is sufficient as input data has len < 100
#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct SeatRow {
    floor: u128,
    occupied: u128,
}

impl SeatRow {
    fn is_floor(&self, idx: u8) -> bool {
        self.floor & 1 << idx > 0
    }

    fn is_occupied(&self, idx: u8) -> bool {
        self.occupied & 1 << idx > 0
    }

    // fn is_empty(&self, idx: u8) -> bool {
    //     (self.floor | self.occupied) & 1 << idx == 0
    // }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            self.row_len = line.len() as u8;

            let row = line.trim().char_indices().try_fold(
                SeatRow {
                    floor: 0,
                    occupied: 0,
                },
                |mut acc, (idx, chr)| {
                    match chr {
                        'L' => {}
                        '#' => {
                            acc.occupied |= 1 << idx;
                        }
                        '.' => {
                            acc.floor |= 1 << idx;
                        }
                        _ => {
                            return Err(format!("unsupported character: '{}'", chr));
                        }
                    };
                    Ok(acc)
                },
            );

            match row {
                Ok(r) => {
                    self.seats.push(r);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        })
    }

    fn part_1(&self) -> Result<String, String> {
        let mut seat_container = Container {
            row_len: self.row_len,
            seats: self.seats.clone(),
        };
        for _ in 0..100_000 {
            let mut change_set = Vec::<(usize, bool, u128)>::with_capacity(self.row_len as usize);

            seat_container
                .seats
                .iter()
                .enumerate()
                .for_each(|(idx, seat_row)| {
                    for i in 0..self.row_len {
                        if seat_row.is_floor(i as u8) {
                            continue;
                        }

                        let adjacent = seat_container.adjacent(idx, i);
                        let occupied = seat_row.is_occupied(i as u8);

                        if occupied && adjacent >= 4 {
                            change_set.push((idx, true, !(1u128 << i)));
                        } else if !occupied && adjacent == 0 {
                            change_set.push((idx, false, 1 << i));
                        }
                    }
                });

            if change_set.is_empty() {
                return Ok(seat_container
                    .seats
                    .iter()
                    .fold(0u32, |mut acc, seat_row| {
                        acc += seat_row.occupied.count_ones();
                        acc
                    })
                    .to_string());
            }

            change_set.iter().for_each(|(idx, unset, change)| {
                if *unset {
                    seat_container.seats[*idx].occupied &= change;
                } else {
                    seat_container.seats[*idx].occupied |= change;
                }
            });
        }
        Err("failed to find stable state after 100,000 iterations".to_owned())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut seat_container = Container {
            row_len: self.row_len,
            seats: self.seats.clone(),
        };
        for _ in 0..100_000 {
            let mut change_set = Vec::<(usize, bool, u128)>::with_capacity(self.row_len as usize);

            seat_container
                .seats
                .iter()
                .enumerate()
                .for_each(|(idx, seat_row)| {
                    for i in 0..self.row_len {
                        if seat_row.is_floor(i as u8) {
                            continue;
                        }

                        let adjacent = seat_container.visibly_adjacent(idx, i as usize);
                        let occupied = seat_row.is_occupied(i as u8);

                        if occupied && adjacent >= 5 {
                            change_set.push((idx, true, !(1u128 << i)));
                        } else if !occupied && adjacent == 0 {
                            change_set.push((idx, false, 1 << i));
                        }
                    }
                });

            if change_set.is_empty() {
                return Ok(seat_container
                    .seats
                    .iter()
                    .fold(0u32, |mut acc, seat_row| {
                        acc += seat_row.occupied.count_ones();
                        acc
                    })
                    .to_string());
            }

            change_set.iter().for_each(|(idx, unset, change)| {
                if *unset {
                    seat_container.seats[*idx].occupied &= change;
                } else {
                    seat_container.seats[*idx].occupied |= change;
                }
            });
        }
        Err("failed to find stable state after 100,000 iterations".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let expected = make_expected();

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example() {
        let input = make_expected();

        let expected = 37.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = make_expected();

        let expected = 26.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    fn make_expected() -> Container {
        Container {
            row_len: 10,
            seats: vec![
                SeatRow {
                    floor: 1 << 1 | 1 << 4 | 1 << 7,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 7,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 1 | 1 << 3 | 1 << 5 | 1 << 6 | 1 << 8 | 1 << 9,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 4 | 1 << 7,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 1 | 1 << 4 | 1 << 7,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 1 | 1 << 7,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 0 | 1 << 1 | 1 << 3 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8 | 1 << 9,
                    occupied: 0,
                },
                SeatRow {
                    floor: 0,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 1 | 1 << 8,
                    occupied: 0,
                },
                SeatRow {
                    floor: 1 << 1 | 1 << 7,
                    occupied: 0,
                },
            ],
        }
    }
}
