use std::{collections::HashSet, str::FromStr};

use aoc::*;

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_once(' ').unwrap();
        let amt = amt.parse::<u32>().unwrap();

        match dir {
            "U" => Ok(Move::Up(amt)),
            "D" => Ok(Move::Down(amt)),
            "R" => Ok(Move::Right(amt)),
            "L" => Ok(Move::Left(amt)),
            _ => Err(()),
        }
    }
}

type Position = (i32, i32);

struct Program {
    tail_pos: Position,
    head_pos: Position,
    tail_visited: HashSet<Position>,
}

impl Program {
    fn new() -> Self {
        Program {
            tail_pos: (0, 0),
            head_pos: (0, 0),
            tail_visited: HashSet::new(),
        }
    }

    fn is_tail_adjacent(&self) -> bool {
        self.tail_pos == self.head_pos
            || ((self.tail_pos.0 - self.head_pos.0).abs() < 2
                && (self.tail_pos.1 - self.head_pos.1).abs() < 2)
    }

    fn get_new_tail_pos(&self) -> Position {
        if !self.is_tail_adjacent() {
            if self.tail_pos.0 != self.head_pos.0 && self.tail_pos.1 != self.head_pos.1 {
                if self.tail_pos.0 > self.head_pos.0 && self.tail_pos.1 < self.head_pos.1 {
                    (self.tail_pos.0 - 1, self.tail_pos.1 + 1)
                } else if self.tail_pos.0 < self.head_pos.0 && self.tail_pos.1 < self.head_pos.1 {
                    (self.tail_pos.0 + 1, self.tail_pos.1 + 1)
                } else if self.tail_pos.0 < self.head_pos.0 && self.tail_pos.1 > self.head_pos.1 {
                    (self.tail_pos.0 + 1, self.tail_pos.1 - 1)
                } else if self.tail_pos.0 > self.head_pos.0 && self.tail_pos.1 > self.head_pos.1 {
                    (self.tail_pos.0 - 1, self.tail_pos.1 - 1)
                } else {
                    unreachable!()
                }
            } else {
                if self.tail_pos.0 == self.head_pos.0 {
                    if self.tail_pos.1 > self.head_pos.1 {
                        (self.tail_pos.0, self.tail_pos.1 - 1)
                    } else if self.tail_pos.1 < self.head_pos.1 {
                        (self.tail_pos.0, self.tail_pos.1 + 1)
                    } else {
                        unreachable!()
                    }
                } else if self.tail_pos.1 == self.head_pos.1 {
                    if self.tail_pos.0 > self.head_pos.0 {
                        (self.tail_pos.0 - 1, self.tail_pos.1)
                    } else if self.tail_pos.0 < self.head_pos.0 {
                        (self.tail_pos.0 + 1, self.tail_pos.1)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
        } else {
            self.tail_pos
        }
    }

    fn simulate(&mut self, moves: &Vec<Move>) {
        moves.iter().for_each(|head_move| {
            match head_move {
                Move::Up(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.1 += 1;

                        let new_tail_pos = self.get_new_tail_pos();

                        self.tail_pos = new_tail_pos;
                        self.tail_visited.insert(new_tail_pos);
                    });
                }
                Move::Down(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.1 -= 1;

                        let new_tail_pos = self.get_new_tail_pos();

                        self.tail_pos = new_tail_pos;
                        self.tail_visited.insert(new_tail_pos);
                    });
                }
                Move::Right(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.0 += 1;

                        let new_tail_pos = self.get_new_tail_pos();

                        self.tail_pos = new_tail_pos;
                        self.tail_visited.insert(new_tail_pos);
                    });
                }
                Move::Left(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.0 -= 1;

                        let new_tail_pos = self.get_new_tail_pos();

                        self.tail_pos = new_tail_pos;
                        self.tail_visited.insert(new_tail_pos);
                    });
                }
            };
        })
    }
}

const TAIL_LENTGH: usize = 9;

#[derive(Debug)]
struct Program2 {
    tail_pos: [Position; TAIL_LENTGH],
    head_pos: Position,
    tail_visited: HashSet<Position>,
}

impl Program2 {
    fn new() -> Self {
        Program2 {
            tail_pos: [(0, 0); TAIL_LENTGH],
            head_pos: (0, 0),
            tail_visited: HashSet::from_iter(vec![(0, 0)]),
        }
    }

    fn is_adjacent(&self, index: usize) -> bool {
        let next_block = if index == 0 {
            self.head_pos
        } else {
            self.tail_pos[index as usize - 1]
        };

        self.tail_pos[index as usize] == next_block
            || ((self.tail_pos[index as usize].0 - next_block.0).abs() < 2
                && (self.tail_pos[index as usize].1 - next_block.1).abs() < 2)
    }

    fn get_new_pos(&self, index: usize) -> Position {
        let next_block = if index == 0 {
            self.head_pos
        } else {
            self.tail_pos[index as usize - 1]
        };

        if !self.is_adjacent(index) {
            if self.tail_pos[index as usize].0 != next_block.0
                && self.tail_pos[index as usize].1 != next_block.1
            {
                if self.tail_pos[index as usize].0 > next_block.0
                    && self.tail_pos[index as usize].1 < next_block.1
                {
                    (
                        self.tail_pos[index as usize].0 - 1,
                        self.tail_pos[index as usize].1 + 1,
                    )
                } else if self.tail_pos[index as usize].0 < next_block.0
                    && self.tail_pos[index as usize].1 < next_block.1
                {
                    (
                        self.tail_pos[index as usize].0 + 1,
                        self.tail_pos[index as usize].1 + 1,
                    )
                } else if self.tail_pos[index as usize].0 < next_block.0
                    && self.tail_pos[index as usize].1 > next_block.1
                {
                    (
                        self.tail_pos[index as usize].0 + 1,
                        self.tail_pos[index as usize].1 - 1,
                    )
                } else if self.tail_pos[index as usize].0 > next_block.0
                    && self.tail_pos[index as usize].1 > next_block.1
                {
                    (
                        self.tail_pos[index as usize].0 - 1,
                        self.tail_pos[index as usize].1 - 1,
                    )
                } else {
                    unreachable!()
                }
            } else {
                if self.tail_pos[index as usize].0 == next_block.0 {
                    if self.tail_pos[index as usize].1 > next_block.1 {
                        (
                            self.tail_pos[index as usize].0,
                            self.tail_pos[index as usize].1 - 1,
                        )
                    } else if self.tail_pos[index as usize].1 < next_block.1 {
                        (
                            self.tail_pos[index as usize].0,
                            self.tail_pos[index as usize].1 + 1,
                        )
                    } else {
                        unreachable!()
                    }
                } else if self.tail_pos[index as usize].1 == next_block.1 {
                    if self.tail_pos[index as usize].0 > next_block.0 {
                        (
                            self.tail_pos[index as usize].0 - 1,
                            self.tail_pos[index as usize].1,
                        )
                    } else if self.tail_pos[index as usize].0 < next_block.0 {
                        (
                            self.tail_pos[index as usize].0 + 1,
                            self.tail_pos[index as usize].1,
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
        } else {
            self.tail_pos[index as usize]
        }
    }

    fn simulate(&mut self, moves: &Vec<Move>) {
        moves.iter().for_each(|head_move| {
            match head_move {
                Move::Up(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.1 += 1;

                        (0..TAIL_LENTGH).for_each(|i| {
                            let new_pos = self.get_new_pos(i);

                            self.tail_pos[i as usize] = new_pos;
                        });

                        self.tail_visited.insert(self.tail_pos[TAIL_LENTGH - 1]);
                    });
                }
                Move::Down(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.1 -= 1;

                        (0..TAIL_LENTGH).for_each(|i| {
                            let new_pos = self.get_new_pos(i);

                            self.tail_pos[i as usize] = new_pos;
                        });

                        self.tail_visited.insert(self.tail_pos[TAIL_LENTGH - 1]);
                    });
                }
                Move::Right(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.0 += 1;

                        (0..TAIL_LENTGH).for_each(|i| {
                            let new_pos = self.get_new_pos(i);

                            self.tail_pos[i as usize] = new_pos;
                        });

                        self.tail_visited.insert(self.tail_pos[TAIL_LENTGH - 1]);
                    });
                }
                Move::Left(a) => {
                    (1..=*a).for_each(|_| {
                        self.head_pos.0 -= 1;

                        (0..TAIL_LENTGH).for_each(|i| {
                            let new_pos = self.get_new_pos(i);

                            self.tail_pos[i as usize] = new_pos;
                        });

                        self.tail_visited.insert(self.tail_pos[TAIL_LENTGH - 1]);
                    });
                }
            };
        })
    }
}

fn main() {
    let aoc = AdventOfCode::new(9, 2022);

    let moves = aoc
        .lines
        .iter()
        .flat_map(|l| l.parse::<Move>())
        .collect::<Vec<Move>>();

    let mut program = Program::new();
    program.simulate(&moves);
    let first_part = program.tail_visited.len();

    let mut program2 = Program2::new();
    program2.simulate(&moves);
    let second_part = program2.tail_visited.len();

    aoc.output(first_part, second_part);
}
