// THIS DAY DOES NOT WORK!

use aoc::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<String>,
    name: String,
    children: Vec<Element>,
}

#[derive(Debug, Clone)]
struct File {
    parent: Option<String>,
    name: String,
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

impl Directory {
    fn root() -> Self {
        Directory {
            parent: None,
            name: String::from("/"),
            children: vec![],
        }
    }

    fn populate(&mut self, elements: &Vec<Element>) {
        elements.iter().for_each(|element| match element {
            Element::Dir(dir) => {
                let parent = dir.parent.clone().unwrap();
                let parent = parent
                    .split('/')
                    .filter(|c| c != &"")
                    .collect::<Vec<&str>>();

                if parent.len() == 0 {
                    self.children.push(Element::Dir(dir.clone()));
                } else {
                    let mut search_dir = self.clone();

                    parent.iter().for_each(|parent_dir| {
                        let the_dir = search_dir.children.iter().find(|d| match d {
                            Element::File(_) => false,
                            Element::Dir(d) => d.name.eq(parent_dir),
                        });

                        let new = if the_dir.is_none() {
                            Directory {
                                children: vec![],
                                name: parent_dir.to_string(),
                                parent: Some(String::from("/")),
                            }
                        } else {
                            match the_dir.unwrap() {
                                Element::Dir(d) => d.clone(),
                                Element::File(_) => unreachable!(),
                            }
                        };

                        search_dir = new.clone();
                    });

                    search_dir.children.push(Element::Dir(dir.clone()));

                    self.children = self
                        .children
                        .iter()
                        .filter(|element| match element {
                            Element::File(_) => true,
                            Element::Dir(d) => d.name != parent[0],
                        })
                        .map(|x| x.clone())
                        .collect();

                    self.children.push(Element::Dir(search_dir.clone()));
                }
            }
            Element::File(file) => {
                let parent = file.parent.clone().unwrap();
                let parent = parent
                    .split('/')
                    .filter(|c| c != &"")
                    .collect::<Vec<&str>>();

                if parent.len() == 0 {
                    self.children.push(Element::File(file.clone()));
                } else {
                    let mut search_dir = self.clone();

                    parent.iter().enumerate().for_each(|(k, parent_dir)| {
                        let the_dir = search_dir.children.iter().find(|d| match d {
                            Element::File(_) => false,
                            Element::Dir(d) => d.name.eq(parent_dir),
                        });

                        let new = if the_dir.is_none() {
                            Directory {
                                children: vec![],
                                name: parent_dir.to_string(),
                                parent: Some(String::from("/")),
                            }
                        } else {
                            match the_dir.unwrap() {
                                Element::Dir(d) => d.clone(),
                                Element::File(_) => unreachable!(),
                            }
                        };

                        search_dir = new.clone();

                        if k == parent.len() - 1 {
                            search_dir.children.push(Element::File(file.clone()));
                        }
                    });

                    self.children = self
                        .children
                        .iter()
                        .filter(|element| match element {
                            Element::File(_) => true,
                            Element::Dir(d) => d.name != parent[0],
                        })
                        .map(|x| x.clone())
                        .collect();

                    self.children.push(Element::Dir(search_dir.clone()));
                }
            }
        });
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parents = self
            .parent
            .as_ref()
            .unwrap_or(&String::from(""))
            .split('/')
            .filter(|c| c != &"")
            .map(|_| "  ")
            .collect::<String>();

        writeln!(f, "{}- {} (dir)", parents, self.name).unwrap();

        for element in &self.children {
            match element {
                Element::Dir(dir) => {
                    let parents = dir
                        .parent
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .split('/')
                        .filter(|c| c != &"")
                        .map(|_| "  ")
                        .collect::<String>();

                    writeln!(f, "  {}{}", parents, dir).unwrap()
                }
                Element::File(file) => {
                    let parents = file
                        .parent
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .split('/')
                        .filter(|c| c != &"")
                        .map(|_| "  ")
                        .collect::<String>();

                    writeln!(f, "  {}- {} (file, size={})", parents, file.name, file.size).unwrap()
                }
            }
        }

        Ok(())
    }
}

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
                            name: name.to_string(),
                            size: value,
                        })),
                        Err(_) => TerminalString::Element(Element::Dir(Directory {
                            parent: None,
                            name: name.to_string(),
                            children: vec![],
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

    let elements = parse_input_to_commands(&aoc.test_lines.as_ref().unwrap())
        .iter()
        .flat_map(|command| match command {
            Command::ChangeDir(_) => None,
            Command::List(output) => Some(output),
        })
        .flat_map(|x| x.clone())
        .collect::<Vec<Element>>();

    let mut root_dir = Directory::root();
    root_dir.populate(&elements);

    debug(&root_dir);
    println!("{}", &root_dir);

    let first_part = 1;

    let second_part = 2;

    aoc.output(first_part, second_part);
}
