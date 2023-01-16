mod variable;

fn main() {
    let grid = generate_grid_from_input(variable::INPUT);
    let result = count_visible_trees(&grid);
    let largest_scenic_number = find_largest_scenic_number(&grid);
    println!("the number of visible trees is {:?}", result);
    println!("the largest scenic number is {:?}", largest_scenic_number);
}

fn find_largest_scenic_number(grid: &Vec<Vec<u32>>) -> i32 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let scenic_number = calcualte_scenic_score(grid, y, x);
            if scenic_number > result {
                result = scenic_number
            }
        }
    }
    result
}

fn generate_grid_from_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_visible_trees(grid: &Vec<Vec<u32>>) -> i32 {
    let mut result = 0;
    for y in 0..grid.len() {
        for i in 0..grid[y].len() {
            let is_top_or_bottom = y == 0 || y == grid.len() - 1;
            let is_left_or_right = i == 0 || i == grid[i].len() - 1;
            if is_left_or_right || is_top_or_bottom || check_if_visible(grid, y, i) {
                result += 1
            }
        }
    }
    result
}

fn check_if_visible(grid: &Vec<Vec<u32>>, row: usize, collum: usize) -> bool {
    let element = grid[row][collum];
    for y in 0..row {
        if element <= grid[y][collum] {
            break;
        }
        if y == row - 1 {
            return true;
        }
    }
    for y in row + 1..grid.len() {
        if element <= grid[y][collum] {
            break;
        }
        if y == grid.len() - 1 {
            return true;
        }
    }
    for x in 0..collum {
        if element <= grid[row][x] {
            break;
        }
        if x == collum - 1 {
            return true;
        }
    }
    for x in collum + 1..grid[collum].len() {
        if element <= grid[row][x] {
            break;
        }
        if x == grid.len() - 1 {
            return true;
        }
    }
    false
}

fn calcualte_scenic_score(grid: &Vec<Vec<u32>>, row: usize, collum: usize) -> i32 {
    let element = grid[row][collum];
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    for y in (0..row).rev() {
        if element > grid[y][collum] {
            a += 1;
        } else {
            a += 1;
            break;
        }
    }
    for y in row + 1..grid.len() {
        if element > grid[y][collum] {
            b += 1;
        } else {
            b += 1;
            break;
        }
    }
    for x in (0..collum).rev() {
        if element > grid[row][x] {
            c += 1;
        } else {
            c += 1;
            break;
        }
    }
    for x in collum + 1..grid[collum].len() {
        if element > grid[row][x] {
            d += 1;
        } else {
            d += 1;
            break;
        }
    }
    a * b * c * d
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let input = "30373
25512
65332
33549
35390";
        let expected = 21;
        let grid = generate_grid_from_input(input);
        let result = count_visible_trees(&grid);
        assert_eq!(result, expected);
    }
    #[test]
    fn scenic_score_first_example() {
        let input = "30373
25512
65332
33549
35390";
        let expected = 4;
        let grid = generate_grid_from_input(input);
        let result = calcualte_scenic_score(&grid, 1, 2);

        assert_eq!(result, expected);
    }
    #[test]
    fn scenic_score_second_example() {
        let input = "30373
25512
65332
33549
35390";
        let expected = 8;
        let grid = generate_grid_from_input(input);
        let result = calcualte_scenic_score(&grid, 3, 2);

        assert_eq!(result, expected);
    }
    #[test]
    fn find_largest_scenic_number_test() {
        let input = "30373
25512
65332
33549
35390";
        let expected = 8;
        let grid = generate_grid_from_input(input);
        let result = find_largest_scenic_number(&grid);

        assert_eq!(result, expected);
    }
}
