use aoc::*;
use pathfinding::prelude::dijkstra;

fn main() {
    let aoc = AdventOfCode::new(12, 2022);

    let input = &aoc.content;

    let mut starting_pos = (0, 0);
    let mut finish_pos = (0, 0);

    let board = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        starting_pos = (y, x);
                        return 'a' as u8;
                    }

                    if c == 'E' {
                        finish_pos = (y, x);
                        return 'z' as u8;
                    }

                    return c as u8;
                })
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let (_, count) = dijkstra(
        &starting_pos,
        |p| {
            let current = board[p.0][p.1];

            let t = if p.0 > 0 { Some((p.0 - 1, p.1)) } else { None };

            let r = if p.1 < board[0].len() - 1 {
                Some((p.0, p.1 + 1))
            } else {
                None
            };

            let b = if p.0 < board.len() - 1 {
                Some((p.0 + 1, p.1))
            } else {
                None
            };

            let l = if p.1 > 0 { Some((p.0, p.1 - 1)) } else { None };

            let successors = vec![t, r, b, l]
                .iter()
                .flat_map(|x| x)
                .filter(|&&x| (0..=current + 1).contains(&board[x.0][x.1]))
                .map(|&x| (x, 1))
                .collect::<Vec<((usize, usize), u32)>>();

            return successors;
        },
        |p| *p == finish_pos,
    )
    .unwrap();

    let first_part = count;

    let second_part = board
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .flat_map(|(x, c)| {
                    if *c == 'a' as u8 {
                        dijkstra(
                            &(y, x),
                            |p| {
                                let current = board[p.0][p.1];

                                let t = if p.0 > 0 { Some((p.0 - 1, p.1)) } else { None };

                                let r = if p.1 < board[0].len() - 1 {
                                    Some((p.0, p.1 + 1))
                                } else {
                                    None
                                };

                                let b = if p.0 < board.len() - 1 {
                                    Some((p.0 + 1, p.1))
                                } else {
                                    None
                                };

                                let l = if p.1 > 0 { Some((p.0, p.1 - 1)) } else { None };

                                let successors = vec![t, r, b, l]
                                    .iter()
                                    .flat_map(|x| x)
                                    .filter(|&&x| (0..=current + 1).contains(&board[x.0][x.1]))
                                    .map(|&x| (x, 1))
                                    .collect::<Vec<((usize, usize), u32)>>();

                                return successors;
                            },
                            |p| *p == finish_pos,
                        )
                    } else {
                        None
                    }
                })
                .map(|x| x.1)
                .collect::<Vec<u32>>()
        })
        .min()
        .unwrap();

    aoc.output(first_part, second_part);
}
