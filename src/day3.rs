struct Diagnostics {
    numbers: Vec<u16>,
    len: usize,
}

#[aoc_generator(day3)]
fn generator_input(input: &str) -> Diagnostics {
    Diagnostics {
        numbers: input.lines().map(|s| u16::from_str_radix(s, 2).unwrap()).collect(),
        len: input.lines().next().unwrap().len(),
    }
}

#[aoc(day3, part1)]
fn part1(input: &Diagnostics) -> u32 {
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in (0..input.len).rev() {
        gamma.push_str(&most_common(&input.numbers, i).to_string());
        epsilon.push_str(&least_common(&input.numbers, i).to_string());
    }
    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &Diagnostics) -> u32 {
    oxygen(most_common, &input.numbers, input.len) as u32 * oxygen(least_common, &input.numbers, input.len) as u32
}

fn oxygen(f: fn(&[u16], usize) -> usize, nums: &[u16], len: usize) -> u16 {
    let mut l: Vec<u16> = nums.to_vec();
    let mut pos: usize = len;
    while l.len() > 1 {
        pos -= 1;
        l = l.iter().filter(|&x| {
            // if 1 is most common, filter out those numbers that don't have 1 on position pos
            if f(&l, pos) == 1 {
                return is_bit_set_at_position(pos, *x);
            }
            !is_bit_set_at_position(pos, *x)
        }).cloned().collect();
    }
    *l.get(0).unwrap()
}

fn is_bit_set_at_position(pos: usize, x: u16) -> bool {
    x & (1 << pos) != 0
}

fn most_common(nums: &[u16], pos: usize) -> usize {
    if count_bits_at_position(nums, pos) >= ((nums.len() + 1) / 2) as u16 {
        return 1;
    }
    0
}

fn least_common(nums: &[u16], pos: usize) -> usize {
    if count_bits_at_position(nums, pos) < ((nums.len() + 1) / 2) as u16 {
        return 1;
    }
    0
}

fn count_bits_at_position(nums: &[u16], pos: usize) -> u16 {
    nums.iter().fold(0_u16, |acc, i| if (i & (1 << pos)) != 0 { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(&generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, part2(&generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")))
    }

    #[test]
    fn test_oxygen() {
        let input = &generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        assert_eq!(23, oxygen(most_common, &input.numbers, input.len));
    }

    #[test]
    fn test_co2() {
        let input = &generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        assert_eq!(10, oxygen(least_common, &input.numbers, input.len));
    }

    #[test]
    fn test_most_common() {
        let input = &generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let expect = vec![1, 0, 1, 1, 0];
        for i in 0..5 {
            assert_eq!(expect[i], most_common(&input.numbers, 4 - i));
        }
    }

    #[test]
    fn test_least_common() {
        let input = &generator_input("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let expect = vec![0, 1, 0, 0, 1];
        for i in 0..5 {
            assert_eq!(expect[i], least_common(&input.numbers, 4 - i));
        }
    }
}

