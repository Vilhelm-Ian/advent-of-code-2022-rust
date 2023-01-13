mod variable;

fn main() {
    let result = find_start_of_packet(variable::INPUT, 14);
    println!("Hello, world! {:?}", result);
}

fn find_start_of_packet(signal: &str, limit: usize) -> Option<i32> {
    let chars: Vec<char> = signal.chars().collect();
    let mut i = 0;
    while i < chars.len() - limit {
        let mut marker = vec![' '; limit];
        for z in 0..limit {
            if marker.contains(&chars[i + z]) {
                break;
            }
            marker[z] = chars[i + z];
            if z == limit - 1 {
                return Some(i as i32 + limit as i32);
            }
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_example() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = 7;
        let result = find_start_of_packet(input, 4).unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn second_example() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let expected = 5;
        let result = find_start_of_packet(input, 4).unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn third_example() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let expected = 6;
        let result = find_start_of_packet(input, 4).unwrap();
        assert_eq!(result, expected);
    }
}
