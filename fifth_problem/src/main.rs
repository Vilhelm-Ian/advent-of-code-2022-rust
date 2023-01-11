mod variable;

struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn move_elements(self, vectors: &mut Vec<Vec<char>>) {
        for _ in 0..self.how_many {
            let last_element = vectors[self.from - 1].pop().unwrap();
            vectors[self.to - 1].push(last_element);
        }
    }
    fn move_elemets_retain_order(self, vectors: &mut Vec<Vec<char>>) {
        let mut elements = vec![' '; self.how_many];
        for i in 0..self.how_many {
            let last_element = vectors[self.from - 1].pop().unwrap();
            elements[self.how_many - 1 - i] = last_element;
        }
        vectors[self.to - 1].append(&mut elements);
    }
    fn new(how_many: usize, from: usize, to: usize) -> Self {
        Self { how_many, from, to }
    }
}

fn main() {
    let input: Vec<&str> = variable::INPUT.split('\n').rev().collect();
    let mut parsed_input: Vec<Vec<char>> = Vec::new();
    for row in input {
        let mut i = 1;
        let chars: Vec<char> = row.chars().collect();
        while i < row.len() {
            let length = parsed_input.len();
            if length == i / 4 {
                parsed_input.push(Vec::new());
            }
            if chars[i] != ' ' {
                parsed_input[i / 4].push(chars[i]);
            }
            i += 4;
        }
    }
    let instructions: Vec<Instruction> = variable::INSTRUCTIONS
        .split('\n')
        .map(|line| {
            let splited_line: Vec<&str> = line.split(' ').collect();
            Instruction::new(
                splited_line[1].parse().unwrap(),
                splited_line[3].parse().unwrap(),
                splited_line[5].parse().unwrap(),
            )
        })
        .collect();
    for instruction in instructions {
        instruction.move_elemets_retain_order(&mut parsed_input);
    }
    let mut result = String::from("");
    for collum in parsed_input {
        let length = collum.len();
        result.push(collum[length - 1]);
    }
    println!("{:?}", result);
}
