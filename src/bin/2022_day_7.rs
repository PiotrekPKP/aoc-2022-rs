use aoc::*;

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<String>,
    name: String,
}

#[derive(Debug, Clone)]
struct File {
    parent: Option<String>,
    size: usize,
}

#[derive(Debug, Clone)]
enum Element {
    File(File),
    Dir(Directory),
}

#[derive(Debug)]
enum Command {
    ChangeDir(String),
    List(Vec<Element>),
}

#[derive(Debug)]
enum TerminalString {
    Command(Command),
    Element(Element),
}

const DISK_SIZE: u32 = 70000000;
const MIN_UNUSED_SIZE: u32 = 30000000;

fn parse_input_to_commands(lines: &Vec<String>) -> Vec<Command> {
    let mut output = lines
        .iter()
        .map(|line| {
            let split_line = line.split(" ").collect::<Vec<&str>>();

            return match split_line[0] {
                "$" => {
                    let command = split_line[1];

                    match command {
                        "cd" => {
                            let argument = split_line[2];
                            return TerminalString::Command(Command::ChangeDir(
                                argument.to_string(),
                            ));
                        }
                        "ls" => TerminalString::Command(Command::List(vec![])),
                        _ => unreachable!(),
                    }
                }
                first_part => {
                    let name = split_line[1];

                    let parsed_first_part = first_part.parse::<usize>();
                    match parsed_first_part {
                        Ok(value) => TerminalString::Element(Element::File(File {
                            parent: None,
                            size: value,
                        })),
                        Err(_) => TerminalString::Element(Element::Dir(Directory {
                            parent: None,
                            name: name.to_string(),
                        })),
                    }
                }
            };
        })
        .rev()
        .scan(Vec::new(), |state, terminal_string| match terminal_string {
            TerminalString::Element(element) => {
                state.push(element);
                return Some(None);
            }
            TerminalString::Command(cmd) => {
                let command_to_return = match cmd {
                    Command::List(_) => Some(Command::List(state.clone())),
                    other_cmd => Some(other_cmd),
                };

                state.clear();

                return Some(command_to_return);
            }
        })
        .flat_map(|x| x)
        .collect::<Vec<Command>>();

    output.reverse();

    let mut current_dir = String::from("/");

    output.iter_mut().for_each(|command| match command {
        Command::ChangeDir(name) => {
            if name != ".." && name != "/" {
                if current_dir.chars().last().unwrap_or('\\') != '/' {
                    current_dir.push_str("/");
                }

                current_dir.push_str(name);
            } else {
                let last_slash = current_dir.rfind('/');

                if let Some(last_slash) = last_slash {
                    if current_dir.len() > 1 {
                        current_dir.replace_range(last_slash.., "");
                    }
                }
            }
        }
        Command::List(output) => output.iter_mut().for_each(|element| match element {
            Element::Dir(dir) => dir.parent = Some(current_dir.to_string()),
            Element::File(file) => file.parent = Some(current_dir.to_string()),
        }),
    });

    return output;
}

fn main() {
    let aoc = AdventOfCode::new(7, 2022);

    let elements = parse_input_to_commands(&aoc.lines)
        .iter()
        .flat_map(|command| match command {
            Command::ChangeDir(_) => None,
            Command::List(output) => Some(output),
        })
        .flat_map(|x| x.clone())
        .collect::<Vec<Element>>();

    let size_of_root_dir = &elements
        .iter()
        .map(|element| match element {
            Element::Dir(_) => 0,
            Element::File(file) => file.size,
        })
        .sum::<usize>();

    let mut sizes_of_dirs = elements
        .iter()
        .map(|element| match element {
            Element::File(_) => 0,
            Element::Dir(directory) => {
                let directory_path = if directory.parent.as_ref().unwrap().len() == 1 {
                    "/".to_string() + directory.name.as_str()
                } else {
                    directory.parent.as_ref().unwrap().clone() + "/" + directory.name.as_str()
                };

                return elements
                    .iter()
                    .filter(|el| match el {
                        Element::Dir(_) => false,
                        Element::File(file) => file
                            .parent
                            .as_ref()
                            .unwrap()
                            .starts_with(directory_path.as_str()),
                    })
                    .map(|el| match el {
                        Element::Dir(_) => 0,
                        Element::File(file) => file.size,
                    })
                    .sum();
            }
        })
        .collect::<Vec<usize>>();

    let first_part = if *size_of_root_dir <= 100000 {
        size_of_root_dir
            + sizes_of_dirs
                .iter()
                .filter(|size| **size <= 100000 && **size > 0)
                .sum::<usize>()
    } else {
        sizes_of_dirs
            .iter()
            .filter(|size| **size <= 100000 && **size > 0)
            .sum::<usize>()
    };

    let free_space = DISK_SIZE - *size_of_root_dir as u32;
    let left_to_delete = MIN_UNUSED_SIZE - free_space;

    sizes_of_dirs.sort();

    let second_part = sizes_of_dirs
        .iter()
        .filter(|size| **size >= left_to_delete as usize)
        .nth(0)
        .unwrap();

    aoc.output(first_part, second_part);
}
