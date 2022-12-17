use aoc::*;
use std::str::Chars;

#[derive(Debug)]
enum Element {
    Number(i32),
    Array(Vec<Element>),
}

fn parse_string(s: &mut Chars) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut buffer = String::new();

    while let Some(c) = s.next() {
        match c {
            c if c.is_digit(10) => buffer.push(c),
            ',' => {
                if !buffer.is_empty() {
                    elements.push(Element::Number(buffer.parse().unwrap()));
                    buffer.clear();
                }
            }
            '[' => elements.push(Element::Array(parse_string(s))),
            ']' => {
                if !buffer.is_empty() {
                    elements.push(Element::Number(buffer.parse().unwrap()));
                    buffer.clear();
                }

                return elements;
            }
            _ => (),
        }
    }

    if !buffer.is_empty() {
        elements.push(Element::Number(buffer.parse().unwrap()));
    }

    elements
}

fn check_order(pair: &(&Element, &Element)) -> bool {
    let mut flag = false;

    debug(format!("Checking {:?}", pair));

    match pair {
        (Element::Number(a), Element::Number(b)) => {
            if b < a {
                flag = true;
            }
        }
        (Element::Number(a), Element::Array(b)) => {
            let a_arr = vec![Element::Number(*a)];

            let mut f = false;

            a_arr.iter().enumerate().for_each(|(i, a_elem)| {
                if i >= b.len() {
                    return;
                } else {
                    f = if f {
                        true
                    } else {
                        check_order(&(a_elem, &b[i]))
                    };
                }
            });

            flag = if flag { true } else { f };
        }
        (Element::Array(a), Element::Number(b)) => {
            let b_arr = vec![Element::Number(*b)];

            let mut f = false;

            a.iter().enumerate().for_each(|(i, a_elem)| {
                if i >= b_arr.len() {
                    return;
                } else {
                    f = if f {
                        true
                    } else {
                        check_order(&(a_elem, &b_arr[i]))
                    };
                }
            });

            flag = if flag { true } else { f };
        }
        (Element::Array(a), Element::Array(b)) => {
            let mut f = false;

            a.iter().enumerate().for_each(|(i, a_elem)| {
                if i >= b.len() {
                    if i == a.len() - 1 {
                        f = true;
                    }
                } else {
                    f = if f {
                        true
                    } else {
                        check_order(&(a_elem, &b[i]))
                    };
                }
            });

            flag = if flag { true } else { f };
        }
    }

    return flag;
}

fn main() {
    let aoc = AdventOfCode::new(13, 2022);

    let input = &aoc.content;

    let parsed_groups = input
        .split("\n\n")
        .map(|group| {
            let mut objects = group.lines();

            let first_str = objects.next().unwrap();
            let mut first_chars = first_str[1..first_str.len() - 1].chars();

            let second_str = objects.next().unwrap();
            let mut second_chars = second_str[1..second_str.len() - 1].chars();

            let first = parse_string(&mut first_chars);
            let second = parse_string(&mut second_chars);

            (first, second)
        })
        .collect::<Vec<(Vec<Element>, Vec<Element>)>>();

    let sum = &parsed_groups
        .iter()
        .enumerate()
        .flat_map(|(i, (a, b))| {
            let mut flag = false;

            a.iter().enumerate().for_each(|(j, a_element)| {
                if j >= b.len() {
                    if j == a.len() - 1 {
                        flag = true;
                    }
                } else {
                    let b_element = &b[j];

                    flag = if flag {
                        true
                    } else {
                        check_order(&(a_element, b_element))
                    };
                }
            });

            debug(format!(
                "Order for pair {} is {}",
                i + 1,
                if flag { "incorrect" } else { "correct" }
            ));

            if flag {
                None
            } else {
                Some(i + 1)
            }
        })
        .sum::<usize>();

    let first_part = &parsed_groups;

    let second_part = 2;

    aoc.output(first_part, second_part);
}
