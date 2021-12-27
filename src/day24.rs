use std::collections::HashMap;

#[aoc_generator(day24)]
fn generator_input(input: &str) -> Vec<(String, char, Option<String>)> {
    input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        (parts.next().unwrap().to_owned(), parts.next().unwrap().chars().next().unwrap(), parts.next().map(|s| s.to_owned()))
    }).collect()
}

#[aoc(day24, part1)]
fn part1(_: &[(String, char, Option<String>)]) -> usize {
    let mut memory: HashMap<char, i64> = HashMap::new();
    memory.insert('x', 0);
    memory.insert('y', 0);
    memory.insert('z', 0);
    memory.insert('w', 0);
    // let mut input: usize = 99999999999999;
    // let mut result: usize = 0;
    // let mut counter = 0;
    // for input in (9999999999999_u64..99999999999999_u64).rev() {
    //     if counter % 1_000_000 == 0 {
    //         println!("{}...", counter);
    //         println!("input: {}...", input);
    //     }
    //     counter += 1;
    let input = 39765432198765;
    //let digits: Vec<_> = input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    // if digits.contains(&0_u32) {
    //     //model number cannot have a 0
    //     continue;
    // }
    //println!("digits: {:?}", digits);
    // let mut i = 0;
    // for l in program {
    //     // if l.1 == 'z' {
    //     //     println!("{:?}", l);
    //     // }
    //     match l.0.as_str() {
    //         "inp" => {
    //             //println!("found input instruction");
    //             memory.insert(l.1, *digits.get(i).unwrap() as i64);
    //             i += 1;
    //         }
    //         "add" => {
    //             //println!("found add instruction");
    //             let tmp = memory.get(&l.1).unwrap();
    //             if let std::result::Result::Ok(x) = l.2.as_ref().unwrap().parse::<i64>() {
    //                 memory.insert(l.1, tmp + x as i64);
    //             } else {
    //                 //println!("char is {}", &l.2.as_ref().unwrap());
    //                 memory.insert(l.1, tmp + memory.get(&l.2.as_ref().unwrap().chars().next().unwrap()).unwrap());
    //             }
    //         }
    //         "mul" => {
    //             //println!("found mul instruction");
    //             let tmp = memory.get(&l.1).unwrap();
    //             if let std::result::Result::Ok(x) = l.2.as_ref().unwrap().parse::<i64>() {
    //                 memory.insert(l.1, tmp * x as i64);
    //             } else {
    //                 memory.insert(l.1, tmp * memory.get(&l.2.as_ref().unwrap().chars().next().unwrap()).unwrap());
    //             }
    //         }
    //         "div" => {
    //             //println!("found div instruction");
    //             let tmp = memory.get(&l.1).unwrap();
    //             if let std::result::Result::Ok(x) = l.2.as_ref().unwrap().parse::<i64>() {
    //                 memory.insert(l.1, tmp / x as i64);
    //             } else {
    //                 memory.insert(l.1, tmp / memory.get(&l.2.as_ref().unwrap().chars().next().unwrap()).unwrap());
    //             }
    //         }
    //         "mod" => {
    //             //println!("found mul instruction");
    //             let tmp = memory.get(&l.1).unwrap();
    //             if let std::result::Result::Ok(x) = l.2.as_ref().unwrap().parse::<i64>() {
    //                 memory.insert(l.1, tmp % x as i64);
    //             } else {
    //                 memory.insert(l.1, tmp % memory.get(&l.2.as_ref().unwrap().chars().next().unwrap()).unwrap());
    //             }
    //         }
    //         "eql" => {
    //             //println!("found eql instruction");
    //             let tmp = memory.get(&l.1).unwrap();
    //             if let std::result::Result::Ok(x) = l.2.as_ref().unwrap().parse::<i64>() {
    //                 if *tmp == x as i64 {
    //                     memory.insert(l.1, 1);
    //                 } else {
    //                     memory.insert(l.1, 0);
    //                 }
    //             } else {
    //                 if memory.get(&l.2.as_ref().unwrap().chars().next().unwrap()).unwrap() == tmp {
    //                     memory.insert(l.1, 1);
    //                 } else {
    //                     memory.insert(l.1, 0);
    //                 }
    //             }
    //         }
    //         &_ => { panic!("invalid instruction: {}", l.0) }
    //     }
    // }
    // if *memory.get(&'z').unwrap() == 0 {
    //     println!("found it: {}", input);
    //     result = input as usize;
    //     break;
    // }
    //input -= 1;
    //     memory.insert('x', 0);
    //     memory.insert('y', 0);
    //     memory.insert('z', 0);
    //     memory.insert('w', 0);
    // }
    println!("x: {} y: {} z: {} w: {}", memory.get(&'x').unwrap(), memory.get(&'y').unwrap(), memory.get(&'z').unwrap(), memory.get(&'w').unwrap());
    //result
    input
}

//99999999999990...
#[test]
fn test_parsing() {
    let input = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
    let parsed = generator_input(input);
    assert_eq!(11, parsed.len());
    assert_eq!("inp", parsed[0].0);
    assert_eq!(std::result::Result::Ok(2), parsed[2].2.as_ref().unwrap().parse::<i32>());
}

#[test]
fn test_part1() {
    let input = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
    let parsed = generator_input(input);
    part1(&parsed);
    part1(&generator_input("inp z
inp x
mul z 3
eql z x"));
}

