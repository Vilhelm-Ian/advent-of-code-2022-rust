use std::cell::RefCell;
use std::rc::Rc;

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
        let mut result;
        if self.is_file {
            return format!("{:?} {:?} ", self.name.clone(), self.value);
        } else {
            result = format!("{} {} \n", self.name.clone(), self.value);
        }
        let inditation = " ".repeat(index);
        for i in 0..self.children.len() {
            let child = Rc::clone(&self.children[i]);
            let child_result = child.borrow().print_structure(index + 1);
            result = format!("{result} {inditation}{child_result} \n",)
        }
        result
    }
}

fn solve(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::default()));
    let lines: Vec<&str> = input.split('\n').collect();
    let mut files_and_folders = vec![];
    let mut is_ls = false;
    let mut current_node = Rc::clone(&root);
    for i in 0..lines.len() {
        let line = lines[i];
        let splited_line: Vec<&str> = line.split(' ').collect();
        if is_ls {
            if line.contains('$') || i == lines.len() - 1 {
                if i == lines.len() - 1 {
                    files_and_folders.push(line)
                }
                let node_copy = Rc::clone(&current_node);
                ls_command(files_and_folders, node_copy);
                files_and_folders = vec![];
                is_ls = false;
            } else {
                files_and_folders.push(line)
            }
        }
        if line.contains("ls") {
            is_ls = true;
            continue;
        }
        if splited_line[1] == "cd" {
            if splited_line[2] == "/" {
                continue;
            }
            if splited_line[2] == ".." {
                let parent_current_node = match &current_node.borrow().parent {
                    Some(parent) => Rc::clone(parent),
                    None => continue,
                };
                current_node = Rc::clone(&parent_current_node);
            } else {
                let current_node_clone = Rc::clone(&current_node);
                let current_node_clone2 = Rc::clone(&current_node);
                for i in 0..current_node_clone.borrow().children.len() {
                    let child_name = current_node_clone2.borrow().children[i]
                        .borrow()
                        .name
                        .clone();
                    if child_name == splited_line[2] {
                        current_node = Rc::clone(&current_node_clone.borrow().children[i]);
                        break;
                    }
                }
            }
        }
    }
    root
}

fn ls_command(input: Vec<&str>, tree: Rc<RefCell<Node>>) {
    for file in input {
        let split: Vec<&str> = file.split(' ').collect();
        let cloned_tree = Rc::clone(&tree);
        if split[0] == "dir" {
            push_node(cloned_tree, String::from(split[1]), 0, false)
        } else {
            let size: i32 = split[0].parse().unwrap();
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

fn main() {
    let input = "$ cd /
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
    let result = solve(input);
    let sum = sum_points_for_nodes(&result);
    let print = result.borrow().print_structure(0);
    println!("{print}");
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = "$ cd /
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
        let result = 95437;
        assert_eq!(result, 4);
    }
}
