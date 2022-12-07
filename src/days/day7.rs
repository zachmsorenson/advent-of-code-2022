
// #[derive(PartialEq)]
// enum Filetype {
//     File,
//     Directory
// }

// struct File<'a> {
//     filetype: Filetype,
//     parent: Option<&'a File<'a>>,
//     children: Option<Vec<File<'a>>>
// }

// impl<'a> File<'a> {
//     fn new(_filetype: Filetype, _parent: Option<&'a File<'a>>) -> File<'a> {
//         let mut children = None;
//         if _filetype == Filetype::Directory {
//             children = Some(Vec::<File>::new());
//         }
//         let mut new = File {
//             filetype: _filetype,
//             parent: _parent,
//             children: children,
//         };

//         new
//     }
// }

// struct Filesystem<'a> {
//     head: &'a File<'a>,
//     cwd: &'a File<'a>,
// }

// impl<'a> Filesystem<'static> {
//     fn init() -> Filesystem<'a> {
//         let root = File {
//             filetype: Filetype::Directory,
//             parent: None,
//             children: Some(Vec::<File>::new()),
//         };

//         let filesystem = Filesystem{head: &root, cwd: &root};
//         filesystem
//     }
// }

struct File<'a> {
    name: String,
    is_dir: bool,
    size: u32,
    parent: Option<&'a File<'a>>,
    children: Option<Vec<&'a File<'a>>>,
}

enum Command {
    Cd,
    Ls,
    File,
    Dir,
}

pub struct Line {
    command: Command,
    arg: Option<String>,
    name: Option<String>,
}

pub fn generator(input: &str) -> Vec<Line> {
    let mut lines = Vec::<Line>::new();
    for line in input.lines().into_iter() {
        let v: Vec<&str> = line.split(' ').collect();

        if v.len() == 3 {
            lines.push(Line {
                command: Command::Cd,
                arg: Some(v[2].to_string()),
                name: None,
            });
        } else {
            match v[0] {
                "$" => {
                    lines.push(Line {
                        command: Command::Ls,
                        arg: None,
                        name: None,
                    });
                },
                "dir" => {
                    lines.push(Line {
                        command: Command::Dir,
                        arg: None,
                        name: Some(v[1].to_string()),
                    });
                },
                _ => {
                    lines.push(Line {
                        command: Command::File,
                        arg: Some(v[0].to_string()),
                        name: Some(v[1].to_string()),
                    });
                },
            }
        }
    }
    lines
}

pub fn part1(input: &[Line]) -> i32 {
    let iter = input.iter();

    let mut root = File {
        name: String::from("/"),
        is_dir: true,
        size: 0,
        parent: None,
        children: Some(Vec::new()),
    };

    let cwd = &root;
    
    let curr_path: Vec<String> = vec![root.name];

    for line in iter {
        match line.command {
            Command::Cd => {

            },
            Command::Ls => {

            },
            Command::File => {

            },
            Command::Dir => {

            },
            _ => unreachable!()
        }
    }
    -1
}

pub fn part2(input: &[Line]) -> i32 {
    -1
}
