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

    match pair {
        (Element::Number(a), Element::Number(b)) => {
            if a > b {
                flag = true;
            }
        }
        (Element::Number(a), Element::Array(b)) => {
            let a_arr = vec![Element::Number(*a)];

            let mut f = false;

            for (i, a_elem) in a_arr.iter().enumerate() {
                if i < b.len() {
                    if check_order(&(a_elem, &b[i])) {
                        f = true;
                        break;
                    }
                }
            }

            flag = f;
        }
        (Element::Array(a), Element::Number(b)) => {
            let b_arr = vec![Element::Number(*b)];

            let mut f = false;

            for (i, a_elem) in a.iter().enumerate() {
                if i < b_arr.len() {
                    if check_order(&(a_elem, &b_arr[i])) {
                        f = true;
                        break;
                    }
                }
            }

            flag = f;
        }
        (Element::Array(a), Element::Array(b)) => {
            let mut f = false;

            for (i, a_elem) in a.iter().enumerate() {
                if i >= b.len() {
                    f = true;
                    break;
                }

                if check_order(&(a_elem, &b[i])) {
                    f = true;
                    break;
                }
            }

            flag = f;
        }
    }

    return flag;
}

fn main() {
    let aoc = AdventOfCode::new(13, 2022);

    let input = &aoc.content;
    //let input = &aoc.test_content.as_ref().unwrap();

    let parsed_groups = input
        .split("\n\n")
        .map(|group| {
            let mut objects = group.lines();

            let mut first_chars = objects.next().unwrap().chars();
            let mut second_chars = objects.next().unwrap().chars();

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

            for (j, a_element) in a.iter().enumerate() {
                if j >= b.len() {
                    flag = true;
                    break;
                }

                if check_order(&(a_element, &b[j])) {
                    flag = true;
                    break;
                }
            }

            debug(format!(
                "Order for pair {} is {}",
                i + 1,
                if flag { "incorrect" } else { "correct" }
            ));

            if flag {
                None
            } else {
                Some(i as i32 + 1)
            }
        })
        .sum::<i32>();

    let first_part = &sum;

    let second_part = 2;

    aoc.output(first_part, second_part);
}
