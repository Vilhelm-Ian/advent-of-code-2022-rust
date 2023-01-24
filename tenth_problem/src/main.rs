mod variable;

enum Instruction {
    Addx(i32),
    Noop,
}

fn main() {
    let register = generate_register(variable::INPUT);
    let signals_of_intrest = vec![20, 60, 100, 140, 180, 220];
    let solution = find_solution(&register, signals_of_intrest);
    let solution_2 = draw_screen(&register);
    println!("The solution is {:?}", solution);
    println!("{}", solution_2);
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let line: Vec<&str> = line.split(' ').collect();
            if line[0] == "noop" {
                return Instruction::Noop;
            }
            Instruction::Addx(line[1].parse().unwrap())
        })
        .collect()
}

fn update_register(register: &mut Vec<i32>, instruction: Instruction) {
    let last_element = register[register.len() - 1];
    match instruction {
        Instruction::Noop => register.push(last_element),
        Instruction::Addx(value) => {
            register.push(last_element);
            register.push(last_element + value);
        }
    }
}

fn add_first_element_register(register: &mut Vec<i32>, instruction: Instruction) {
    match instruction {
        Instruction::Noop => register.push(1),
        Instruction::Addx(value) => {
            register.push(value);
        }
    }
}

fn generate_register(input: &str) -> Vec<i32> {
    let mut register = vec![1];
    let instructions = parse_input(input);
    for instruction in instructions {
        if register.is_empty() {
            add_first_element_register(&mut register, instruction)
        } else {
            update_register(&mut register, instruction)
        }
    }
    register
}

fn find_solution(register: &Vec<i32>, intresting_signals: Vec<i32>) -> i32 {
    let mut result = 0;
    for intresting_signal in intresting_signals {
        result += register[(intresting_signal - 1) as usize] * intresting_signal
    }
    result
}

fn draw_screen(register: &Vec<i32>) -> String {
    let mut result = String::from("");
    for (i, sprite) in register.iter().enumerate() {
        let index = i % 40;
        if (index as i32) - sprite < 2 && (index as i32) - sprite > -2 {
            result += "#"
        } else {
            result += "."
        }
        if (i + 1) / 40 == 6 {
            break;
        }
        if (index + 1) % 40 == 0 {
            result += "\n";
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let vector = generate_register(variable::TEST_DATA);
        let intresting_signals = vec![20, 60, 100, 140, 180, 220];
        let solution = find_solution(vector, intresting_signals);
        //println!("the 20th cycle is: {:?}", vector[21]);
        let expceted = 13140;
        assert_eq!(expceted, solution);
    }
    #[test]
    fn second_test() {
        let input = "noop
addx 3
addx -5";
        let vector = generate_register(input);
        let expceted = -1;
        assert_eq!(expceted, vector[4]);
    }
    #[test]
    fn print() {
        let input = variable::TEST_DATA;
        let vector = generate_register(input);
        let result = draw_screen(vector);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(expected, result);
    }
}
