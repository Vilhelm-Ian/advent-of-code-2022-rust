use std::cell::RefCell;

use std::rc::Rc;
mod variable;

struct File {
    value: i32,
    name: String, //It's not used but useful for debugging
}

impl File {
    fn new(value: i32, name: String) -> Self {
        Self { value, name }
    }
}

struct Node {
    children: Vec<Rc<RefCell<Node>>>,
    value: i32,
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    is_file: bool,
}

impl Node {
    fn new(name: String, parent: Option<Rc<RefCell<Node>>>, value: i32, is_file: bool) -> Self {
        Self {
            value,
            children: Vec::new(),
            parent,
            name,
            is_file,
        }
    }
    fn default() -> Self {
        Self {
            value: 0,
            children: Vec::new(),
            parent: None,
            name: String::from("/"),
            is_file: false,
        }
    }
    fn print_structure(&self, index: usize) -> String {
        let mut result = format!("{} {}", self.name.clone(), self.value);
        let inditation = " ".repeat(index);
        for i in 0..self.children.len() {
            let child = Rc::clone(&self.children[i]);
            let child_result = child.borrow().print_structure(index + 1);
            result = format!("{result} \n {inditation}-{child_result}",)
        }
        result
    }
}

fn main() {
    let input = variable::INPUT;
    let tree = parse_input(input);
    sum_points_for_nodes(&tree);
    //let print = tree.borrow().print_structure(0);
    // println!("{print}");
    let directories = get_all_directories(tree).unwrap();
    let sum_of_directoris = sum_of_directoriese_smaller_than_limit(&directories, 100000);
    let directory_to_delete = find_smallest_directory_above_limit(&directories, 8381165);
    println!("the solution is {:?}", sum_of_directoris);
    println!(
        "the size of the smallest file above limit is {:?}",
        directory_to_delete
    );
}

fn parse_input(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::default()));
    let lines: Vec<&str> = input.split('\n').collect();
    let mut files_and_folders = vec![];
    let mut is_ls = false;
    let mut current_node = Rc::clone(&root);
    for i in 0..lines.len() {
        let line = lines[i];
        let splited_line: Vec<&str> = line.split(' ').collect();
        if is_ls {
            if splited_line[0] != "$" {
                files_and_folders.push(line)
            }
            if splited_line[0] == "$" || i == lines.len() - 1 {
                let node_copy = Rc::clone(&current_node);
                ls_command(files_and_folders, node_copy);
                files_and_folders = vec![];
                is_ls = false;
            }
        }
        if splited_line[1] == "ls" {
            is_ls = true;
        }
        //is_ls = splited_line[1] == "ls" || is_ls; Is this better than the above code
        if splited_line[1] == "cd" {
            let node_copy = Rc::clone(&current_node);
            current_node = match splited_line[2] {
                "/" => continue,
                ".." => move_up_directory(node_copy, i),
                _ => cd_into(node_copy, splited_line[2]).unwrap(),
            };
        }
    }
    root
}

fn move_up_directory(current_node: Rc<RefCell<Node>>, i: usize) -> Rc<RefCell<Node>> {
    let parent_current_node = match &current_node.borrow().parent {
        Some(parent) => Rc::clone(parent),
        None => panic!("has no parent at line {:?}", i),
    };
    Rc::clone(&parent_current_node)
}

fn cd_into(current_node: Rc<RefCell<Node>>, folder_name: &str) -> Option<Rc<RefCell<Node>>> {
    let current_node_clone = Rc::clone(&current_node);
    let current_node_clone2 = Rc::clone(&current_node);
    for i in 0..current_node_clone.borrow().children.len() {
        let child_name = current_node_clone2.borrow().children[i]
            .borrow()
            .name
            .clone();
        if child_name == folder_name {
            return Some(Rc::clone(&current_node_clone.borrow().children[i]));
        }
    }
    None
}

fn ls_command(input: Vec<&str>, tree: Rc<RefCell<Node>>) {
    for file in input {
        let split: Vec<&str> = file.split(' ').collect();
        let cloned_tree = Rc::clone(&tree);
        if split[0] == "dir" {
            push_node(cloned_tree, String::from(split[1]), 0, false)
        } else {
            let size: i32 = match split[0].parse() {
                Ok(number) => number,
                Err(_err) => panic!("failei parisng {:?}", split[0]),
            };
            push_node(cloned_tree, String::from(split[1]), size, true)
        }
    }
}

fn push_node(tree: Rc<RefCell<Node>>, name: String, value: i32, is_file: bool) {
    let tree_clone = Rc::clone(&tree);
    let child = Node::new(name, Some(tree_clone), value, is_file);
    let tree_clone_2 = Rc::clone(&tree);
    tree_clone_2
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(child)));
}

fn sum_points_for_nodes(tree: &Rc<RefCell<Node>>) -> i32 {
    let mut tree_borrowed = tree.borrow_mut();
    if tree_borrowed.is_file {
        return tree_borrowed.value;
    }
    let mut result = tree_borrowed.value;
    for i in 0..tree_borrowed.children.len() {
        let sum_of_child_nodes = sum_points_for_nodes(&tree_borrowed.children[i]);
        tree_borrowed.value += sum_of_child_nodes;
        result += sum_of_child_nodes;
    }
    result
}

fn get_all_directories(tree: Rc<RefCell<Node>>) -> Option<Vec<File>> {
    let tree_borrowed = tree.borrow();
    let mut result: Vec<File> = vec![];
    if tree_borrowed.is_file {
        return None;
    }
    for i in 0..tree_borrowed.children.len() {
        let child = Rc::clone(&tree_borrowed.children[i]);
        let child_clone = tree_borrowed.children[i].borrow();
        if !child_clone.is_file {
            result.push(File::new(child_clone.value, child_clone.name.clone()));
            if let Some(mut directories) = get_all_directories(child) {
                result.append(&mut directories)
            }
        }
    }
    Some(result)
}

fn sum_of_directoriese_smaller_than_limit(directories: &Vec<File>, limit: i32) -> i32 {
    let mut result = 0;
    for directory in directories {
        if directory.value <= limit {
            result += directory.value
        }
    }
    result
}

fn find_smallest_directory_above_limit(directories: &Vec<File>, limit: i32) -> i32 {
    let mut result = 0;
    for directory in directories {
        if directory.value >= limit && (directory.value < result || result == 0) {
            println!("{:?}", directory.value);
            result = directory.value
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_sum() {
        let tree = parse_input(INPUT);
        sum_points_for_nodes(&tree);
        let directories = get_all_directories(tree).unwrap();
        let result = sum_of_directoriese_smaller_than_limit(&directories, 100000);

        let expected = 95437;
        assert_eq!(expected, result);
    }
    #[test]
    fn test_find_smallest_above_limit() {
        let tree = parse_input(INPUT);
        sum_points_for_nodes(&tree);
        let directories = get_all_directories(tree).unwrap();
        let result = find_smallest_directory_above_limit(&directories, 100000);

        let expected = 24933642;
        assert_eq!(expected, result);
    }
}
