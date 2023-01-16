mod variable;

fn main() {
    let result = count_visible_trees(variable::INPUT);
    println!("the number of visible trees is {:?}", result);
}

fn count_visible_trees(input: &str) -> i32 {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();
    let mut result = 0;
    for y in 0..grid.len() {
        for i in 0..grid[y].len() {
            let is_top_or_bottom = y == 0 || y == grid.len() - 1;
            let is_left_or_right = i == 0 || i == grid[i].len() - 1;
            if is_left_or_right || is_top_or_bottom || check_if_visible(&grid, y, i) {
                if !is_left_or_right && !is_top_or_bottom && check_if_visible(&grid, y, i) {
                    println!(
                        "the element {:?} is visible y: {:?}, x: {:?}",
                        grid[y][i], y, i
                    );
                }
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
        let result = count_visible_trees(input);
        assert_eq!(result, expected);
    }
}
