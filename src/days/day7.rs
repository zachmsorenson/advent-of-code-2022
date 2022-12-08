use std::{rc::Rc, cell::RefCell, collections::VecDeque};


fn calculate_size(f: &Rc<RefCell<File>>) -> u32 {
    let f = f.borrow();

    if !f.is_dir {
        return f.size
    }

    let mut sum = 0;
    for child in f.children.as_ref().unwrap() {
        let size;
        {
            size = calculate_size(child);
        }

        let mut borrow = child.borrow_mut();
        if borrow.size == 0 {
            borrow.size = size;
        }

        sum += size;
    }
    sum
}

pub struct File {
    name: String,
    is_dir: bool,
    size: u32,
    children: Option<Vec<Rc<RefCell<File>>>>,
}

pub fn generator(input: &str) -> Rc<RefCell<File>> {
    let root = File {
        name: "/".to_string(),
        is_dir: true,
        size: 0,
        children: Some(Vec::new()),
    };
    let root = Rc::new(RefCell::new(root));

    let mut curr_path: Vec<Rc<RefCell<File>>> = vec![root];

    let input: Vec<&str> = input.lines().collect();
    for &line in input.iter().skip(1) {
        let args: Vec<&str> = line.split(' ').collect();

        match args[0] {
            "$" => {
                match args[1] {
                    "cd" => {
                        match args[2] {
                            ".." => {
                                // cd ..
                                curr_path.pop();
                            },
                            _ => {
                                // cd some_dir
                                let f = curr_path.last().cloned().unwrap();
                                let borrow = f.borrow();
                
                                match borrow.children.as_ref() {
                                    Some(v) => {
                                        let f = v.iter()
                                         .find(|&c| c.borrow_mut().name == args[2].to_string())
                                         .unwrap();

                                        curr_path.push(f.clone());

                                    },
                                    _ => unreachable!(),
                                };
                                drop(borrow);
                            }
                        }
                    },
                    "ls" => {
                        
                    },
                    _ => unreachable!()
                }
            },
            "dir" => {
                let f = curr_path.last().unwrap();
                let mut borrow = f.borrow_mut();

                let new_file = File {
                    name: args[1].to_string(),
                    is_dir: true,
                    size: 0,
                    children: Some(Vec::new()),
                };

                borrow.children.as_mut()
                        .expect("")
                        .push(Rc::new(RefCell::new(new_file)));
                drop(borrow);
            },
            _ => {
                let f = curr_path.last().unwrap();
                let mut borrow = f.borrow_mut();

                let new_file = File {
                    name: args[1].to_string(),
                    is_dir: false,
                    size: args[0].parse().unwrap(),
                    children: None,
                };

                borrow.children.as_mut()
                        .expect("")
                        .push(Rc::new(RefCell::new(new_file)));

                drop(borrow);
            }
        }
    }
    let head = curr_path.first().unwrap();
    let total = calculate_size(head);
    head.borrow_mut().size = total;
    head.clone()
}

pub fn part1(head: &Rc<RefCell<File>>) -> u32 {
    let mut sum = 0;

    let mut queue = VecDeque::new();
    queue.push_back(head.clone());

    while let Some(next) = queue.pop_front() {
        let f = next.borrow();

        if f.is_dir {
            if f.size <= 100000 {
                sum += f.size;
            }

            // let children = f.children.borrow();
            if let Some(children) = &f.children {
                for child in children {
                    queue.push_back(child.clone());
                }
            }
        }
    }
    sum
}

pub fn part2(head: &Rc<RefCell<File>>) -> u32 {
    let mut smallest = 70000000;
    let size = head.borrow().size;
    let required = 30000000 - (smallest - size);

    let mut queue = VecDeque::new();
    queue.push_back(head.clone());

    while let Some(next) = queue.pop_front() {
        let f = next.borrow();

        if f.is_dir {
            if f.size > required && f.size < smallest {
                smallest = f.size;
            }

            // let children = f.children.borrow();
            if let Some(children) = &f.children {
                for child in children {
                    queue.push_back(child.clone());
                }
            }
        }
    }

    smallest
}
