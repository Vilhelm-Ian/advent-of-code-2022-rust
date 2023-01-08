mod variable;

fn main() {
    let parse_input: Vec<&str> = variable::INPUT.split('\n').collect();
    let parse_input: Vec<Vec<&str>> = parse_input.iter().map(|x| x.split(' ').collect()).collect();
    let mut result = 0;
    // X lose, Y draw, Z win
    // A rock
    // B paper
    // C scissors
    for turn in parse_input.iter() {
        let response = find_appropriate_response(turn[0], turn[1]).unwrap();

        let mut score = match (turn[0], response) {
            ("A", "A") | ("B", "B") | ("C", "C") => 3,
            ("A", "C") | ("B", "A") | ("C", "B") => 0,
            _ => 6,
        };
        score += match response {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        };
        result += score;
    }
    println!("{:?}", result);
}

fn find_appropriate_response<'a>(opponent_move: &'a str, result: &'a str) -> Option<&'a str> {
    if result == "Y" {
        return Some(opponent_move);
    }
    if result == "Z" {
        return match opponent_move {
            "A" => Some("B"),
            "B" => Some("C"),
            "C" => Some("A"),
            _ => None,
        };
    }
    match opponent_move {
        "A" => Some("C"),
        "B" => Some("A"),
        "C" => Some("B"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_draw() {
        let result = find_appropriate_response("A", "Y");
        assert_eq!(result.unwrap(), "A");
    }

    #[test]
    fn you_lose() {
        let result = find_appropriate_response("B", "X");
        assert_eq!(result.unwrap(), "A");
    }
    #[test]
    fn you_win() {
        let result = find_appropriate_response("C", "Z");
        assert_eq!(result.unwrap(), "A");
    }
}
