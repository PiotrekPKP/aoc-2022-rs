use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(8, 2022);

    let trees = &aoc
        .lines
        .iter()
        .map(|line| {
            line.chars()
                .flat_map(|c| String::from(c).parse::<u8>())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let first_part = &trees
        .iter()
        .enumerate()
        .map(|(row, row_of_trees)| {
            row_of_trees
                .iter()
                .enumerate()
                .filter(|(column, tree_height)| {
                    if row == 0
                        || *column == 0
                        || row == trees.len() - 1
                        || *column == row_of_trees.len() - 1
                    {
                        return true;
                    }

                    let visible_from_top = (0..row)
                        .map(|i| trees[i][*column] < **tree_height)
                        .all(|v| v == true);

                    let visible_from_right = (*column + 1..row_of_trees.len())
                        .map(|i| trees[row][i] < **tree_height)
                        .all(|v| v == true);

                    let visible_from_bottom = (row + 1..trees.len())
                        .map(|i| trees[i][*column] < **tree_height)
                        .all(|v| v == true);

                    let visible_from_left = (0..*column)
                        .map(|i| trees[row][i] < **tree_height)
                        .all(|v| v == true);

                    return visible_from_top
                        || visible_from_right
                        || visible_from_bottom
                        || visible_from_left;
                })
                .map(|(_, t)| t)
                .count()
        })
        .sum::<usize>();

    let second_part = &trees
        .iter()
        .enumerate()
        .flat_map(|(row, row_of_trees)| {
            row_of_trees
                .iter()
                .enumerate()
                .map(|(column, tree_height)| {
                    if row == 0
                        || column == 0
                        || row == trees.len() - 1
                        || column == row_of_trees.len() - 1
                    {
                        return 0;
                    }

                    let top_score = row
                        - (0..row)
                            .filter(|i| trees[*i][column] >= *tree_height)
                            .last()
                            .unwrap_or(0);

                    let left_score = column
                        - (0..column)
                            .filter(|i| trees[row][*i] >= *tree_height)
                            .last()
                            .unwrap_or(0);

                    let right_score = (column + 1..row_of_trees.len())
                        .filter(|i| trees[row][*i] >= *tree_height)
                        .nth(0)
                        .unwrap_or(row_of_trees.len() - 1)
                        - column;

                    let bottom_score = (row + 1..trees.len())
                        .filter(|i| trees[*i][column] >= *tree_height)
                        .nth(0)
                        .unwrap_or(trees.len() - 1)
                        - row;

                    return top_score * left_score * right_score * bottom_score;
                })
                .collect::<Vec<usize>>()
        })
        .max()
        .unwrap();

    aoc.output(first_part, second_part);
}
