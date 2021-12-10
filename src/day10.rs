use std::collections::HashMap;
use bimap::BiMap;

#[aoc_generator(day10)]
fn generator_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<char>]) -> u32 {
    input.iter().flat_map(move |c| syntax_error(&to_str_slice(c))).sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<char>]) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter(|l| syntax_error(&to_str_slice(l)) == None)
        .map(|l| calc_score(&autocomplete(&to_str_slice(l)))).collect();
    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

// find matching opening paren to the last closing one in the string slice
fn find_matching(line: &str) -> Option<&str> {
    let closing = line.chars().last().unwrap();
    let tbl = table();
    let opening = tbl.get_by_left(&closing).unwrap();
    let mut stack: Vec<char> = vec!();
    for (i, c) in line.chars().rev().enumerate().skip(1) {
        if c != *opening && c != closing {
            continue;
        }
        if c == closing {
            stack.push(c);
        } else if c == *opening {
            if stack.is_empty() {
                let start = line.len() - i;
                return Some(&line[start..line.len() - 1]);
            } else {
                stack.pop();
            }
        }
    }
    None
}

fn to_str_slice(l: &[char]) -> String {
    l.iter().collect::<String>()
}

fn table() -> BiMap<char, char> {
    let mut table = BiMap::new();
    table.insert(')', '(');
    table.insert(']', '[');
    table.insert('}', '{');
    table.insert('>', '<');
    table
}

fn valid_chunk(chuck: &str) -> bool {
    let mut stack: Vec<char> = vec!();
    for c in chuck.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                if let Some(i) = stack.iter().rposition(|x| x == table().get_by_left(&c).unwrap()) {
                    stack.remove(i);
                } else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

fn syntax_error(line: &str) -> Option<u32> {
    let mut table = HashMap::new();
    table.insert(')', 3);
    table.insert(']', 57);
    table.insert('}', 1197);
    table.insert('>', 25137);

    for (i, c) in line.chars().enumerate() {
        match c {
            ')' | ']' | '}' | '>' => {
                match find_matching(&line[0..i + 1]) {
                    Some(chuck) => {
                        if !valid_chunk(chuck) {
                            return table.get(&c).copied();
                        }
                    }
                    None => return table.get(&c).copied()
                }
            }
            _ => {}
        }
    }
    None
}


fn calc_score(completion: &str) -> u64 {
    completion.chars().fold(0, |mut acc, c| {
        acc *= 5;
        acc += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(""),
        };
        acc
    })
}

fn autocomplete(line: &str) -> String {
    let mut stack: Vec<char> = vec![];
    let tbl = table();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => { stack.remove(stack.iter().rposition(|x| x == tbl.get_by_left(&c).unwrap()).expect("should not happen")); }
        }
    }
    stack.clone().iter().flat_map(|p| tbl.get_by_right(p)).rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_parsing() {
        let parsed = generator_input(INPUT);
        assert_eq!(10, parsed.len());
        assert_eq!('[', *parsed.get(0).unwrap().get(0).unwrap());
        assert_eq!('(', *parsed.get(0).unwrap().get(1).unwrap());
    }

    #[test]
    fn test_syntax_error() {
        assert_eq!(None, syntax_error("[<>({}){}[([])<>]]"));
        assert_eq!(Some(1197), syntax_error("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(Some(3), syntax_error("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(Some(57), syntax_error("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(Some(3), syntax_error("[<(<(<(<{}))><([]([]()"));
        assert_eq!(Some(25137), syntax_error("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn test_find_matching() {
        assert_eq!(Some("([(<{}[<>[]"), find_matching("{([(<{}[<>[]}"));
        assert_eq!(None, find_matching("[[<[([]))"));
        assert_eq!(Some(""), find_matching("((((((((()"))
    }

    #[test]
    fn test_valid_chuck() {
        assert!(valid_chunk("([])"));
        assert!(valid_chunk("{()()()}"));
        assert!(valid_chunk("<([{}])>"));
        assert!(valid_chunk("[<>({}){}[([])<>]]"));
        assert!(valid_chunk("(((((((((())))))))))"));
        assert!(!valid_chunk("([(<{}[<>[]"));
        assert!(!valid_chunk("(]"));
        assert!(!valid_chunk("{()()()>"));
        assert!(!valid_chunk("(((()))}"));
        assert!(!valid_chunk("<([]){()}[{}])"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1(&generator_input(INPUT)));
    }

    #[test]
    fn test_autocomplete() {
        assert_eq!("}}]])})]", autocomplete("[({(<(())[]>[[{[]{<()<>>"));
    }

    #[test]
    fn test_score() {
        assert_eq!(294, calc_score("])}>"));
    }


    #[test]
    fn test_part2() {
        assert_eq!(288957, part2(&generator_input(INPUT)));
    }
}