mod variable;

fn main() {
    let result = find_priorities_of_badges(variable::INPUT);
    println!("{:?}", result);
}

fn find_priorities_of_badges(input: &str) -> u32 {
    let splited: Vec<&str> = input.split('\n').collect();
    let mut i = 0;
    let mut result = 0;
    while i < splited.len() {
        let badge = find_badge(splited[i], splited[i + 1], splited[i + 2]);
        let badge_priority = calculate_priority(badge.unwrap());
        result += badge_priority;
        i += 3;
    }
    result
}

fn find_solution(input: &str) -> u32 {
    let mut result = 0;
    for line in input.split('\n') {
        let (split1, split2) = line.split_at(line.len() / 2);
        let items = find_item_in_both_compartments(split1, split2);
        result += items.iter().fold(0, |acc, x| acc + calculate_priority(*x));
    }
    result
}

fn calculate_priority(c: char) -> u32 {
    let ascii = c as u32;
    if ascii > 96 {
        return ascii - 96;
    }
    ascii - 38
}

fn find_item_in_both_compartments(compartment1: &str, compartment2: &str) -> Vec<char> {
    let mut result = vec![];
    for char1 in compartment1.chars() {
        if !result.contains(&char1) && compartment2.contains(char1) {
            result.push(char1)
        }
    }
    result
}

fn find_badge(ruck_sack_1: &str, ruck_sack_2: &str, ruck_sack_3: &str) -> Option<char> {
    let item_in_both_compartments = find_item_in_both_compartments(ruck_sack_1, ruck_sack_2);
    for letter in item_in_both_compartments {
        if ruck_sack_3.contains(letter) {
            return Some(letter);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_p() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (split1, split2) = input.split_at(input.len() / 2);
        let result = find_item_in_both_compartments(split1, split2);
        let expected = vec!['p'];
        assert_eq!(result, expected);
    }
    #[test]
    fn find_L() {
        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let (split1, split2) = input.split_at(input.len() / 2);
        let result = find_item_in_both_compartments(split1, split2);
        let expected = vec!['L'];
        assert_eq!(result, expected);
    }
    #[test]
    fn find_P() {
        let input = "PmmdzqPrVvPwwTWBwg";
        let (split1, split2) = input.split_at(input.len() / 2);
        let result = find_item_in_both_compartments(split1, split2);

        let expected = vec!['P'];
        assert_eq!(result, expected);
    }
    #[test]
    fn p_priority() {
        let result = calculate_priority('p');
        let expected = 16;
        assert_eq!(result, expected);
    }
    #[test]
    fn L_priority() {
        let result = calculate_priority('L');
        let expected = 38;
        assert_eq!(result, expected);
    }
    #[test]
    fn P_priority() {
        let result = calculate_priority('P');
        let expected = 42;
        assert_eq!(result, expected);
    }
    fn v_priority() {
        let result = calculate_priority('v');
        let expected = 22;
        assert_eq!(result, expected);
    }
    #[test]
    fn find_solution_example() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = find_solution(input);
        let expected = 157;
        assert_eq!(result, expected);
    }
    #[test]
    fn find_badge_r() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let ruck_sacks: Vec<&str> = input.split("\n").collect();
        let result = find_badge(ruck_sacks[0], ruck_sacks[1], ruck_sacks[2]);
        let expected = Some('r');
        assert_eq!(result, expected);
    }
}
