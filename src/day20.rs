use array2d::Array2D;

#[aoc_generator(day20)]
fn generator_input(input: &str) -> (Vec<bool>, Array2D<u8>) {
    let mut parts = input.split("\n\n");
    (parts.next().unwrap().chars().map(|c| c == '#').collect(),
     Array2D::from_row_major(&*parts.next().unwrap().lines().flat_map(|c| {
         c.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<u8>>()
     }).collect::<Vec<u8>>(), 5, 5))
}

#[aoc(day20, part1)]
fn part1(input: &(Vec<bool>, Array2D<u8>)) -> usize {
    let output = enhance_image(&input.0, &input.1, false);
    let default_off = input.0.get(0).unwrap();
    println!("default_off: {}", default_off);
    let output = enhance_image(&input.0, &output, *default_off);
    output.elements_row_major_iter().filter(|&el| *el == 1).count()
}

#[aoc(day20, part2)]
fn part2(input: &(Vec<bool>, Array2D<u8>)) -> usize {
    let mut rest = false;
    let mut output = enhance_image(&input.0, &input.1, rest);
    for _ in 0..49 {
        if rest {
            rest = *input.0.get(2_u16.pow(9) as usize - 1).unwrap();
        } else {
            rest = *input.0.get(0).unwrap();
        }
        //println!("rest: {}", rest);
        output = enhance_image(&input.0, &output, rest);
    }
    output.elements_row_major_iter().filter(|&el| *el == 1).count()
}

//returns 2D array that is larger than the original by 2 pixels on each side
fn enhance_image(alg: &[bool], image: &Array2D<u8>, rest: bool) -> Array2D<u8> {
    let new_size = image.num_rows() + 4;
    let mut output: Vec<u8> = vec![0; new_size * new_size];
    let mut idx = 0;
    for r in -2_i32..(image.num_rows() + 2) as i32 {
        for c in -2_i32..(image.num_columns() + 2) as i32 {
            if *alg.get(index(&(r as i32, c as i32), image, rest) as usize).unwrap() {
                output[idx] = 1;
            }
            idx += 1;
        }
    }
    //println!("{:?}", output);
    Array2D::from_row_major(&output, new_size, new_size)
}

#[test]
fn test_parsing() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let parsed = generator_input(input);
    assert_eq!(512, parsed.0.len());
    assert!(!parsed.0[0]);
    assert!(!parsed.0[1]);
    assert!(parsed.0[2]);
    assert_eq!(5, parsed.1.num_rows());
    println!("{:?}", parsed.1);
}

#[test]
fn test_conversion() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let parsed = generator_input(input);
    let image = parsed.1;
    assert_eq!(34, index(&(2, 2), &image, false));
    assert_eq!(18, index(&(0, 0), &image, false));
    assert_eq!(502, index(&(0, 0), &image, true));
}

fn index(pixel: &(i32, i32), image: &Array2D<u8>, rest: bool) -> u16 {
    let mut cnt: i8 = 8;
    let mut result: u16 = 0;
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if pixel.0 + x >= 0 && pixel.0 + x < image.num_columns() as i32 && pixel.1 + y >= 0 && pixel.1 + y < image.num_rows() as i32 {
                let tmp = image[((pixel.0 + x) as usize, (pixel.1 + y) as usize)] as u16;
                result += tmp << cnt;
                //println!("{}-{} is {}",x,y,result);
            } else if rest {
                result += 1 << cnt;
            }
            cnt -= 1;
        }
    }
    result
}

#[test]
fn test_part1() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    assert_eq!(35, part1(&generator_input(input)));
    assert_eq!(3351, part2(&generator_input(input)));
}