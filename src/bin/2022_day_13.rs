use aoc::*;
use regex::Regex;

#[derive(Debug)]
enum Element {
    Number(i32),
    Array(Vec<Element>),
}

fn parse_element(str: &str) -> Element {
    debug(str);
    Element::Array(vec![])
}

fn split_string(str: &str) -> Vec<&str> {
    let re = Regex::new(r",([^\[\]]*\])").unwrap();
    return re.split(str).collect();
}

fn main() {
    let aoc = AdventOfCode::new(13, 2022);

    let elements = split_string("[1],[2,3,4],1,3,[2,[3],4,5],[[],[[]]]");
    let mut result = Vec::new();
    for element in elements {
        if element.starts_with("[") && element.ends_with("]") {
            let inner_elements = element[1..element.len() - 1]
                .split(",")
                .collect::<Vec<&str>>();
            let mut inner_result = Vec::new();
            for inner_element in inner_elements {
                let parsed_inner_element = parse_element(inner_element);
                inner_result.push(parsed_inner_element);
            }
            result.push(Element::Array(inner_result));
        } else {
            debug(element);
            let parsed_number = element.parse::<i32>().unwrap();
            result.push(Element::Number(parsed_number));
        }
    }

    let first_part = "[[1],[2,3,4],1,3,[2,[3],4,5],[[],[[]]]]";

    let second_part = 2;

    aoc.output(first_part, second_part);
}
