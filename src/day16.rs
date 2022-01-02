use nom::combinator::cond;
use nom::multi::count;
use nom::sequence::tuple;
use nom::{bits::complete::take, IResult};

//use nom::bits::bits;

#[aoc_generator(day16)]
fn generator_input(input: &str) -> Vec<u8> {
    hex::decode(input).unwrap()
}

#[derive(Debug)]
pub struct Header {
    ver: u8,
    type_id: u8,
}

#[aoc(day16, part1)]
fn part1(bits: &[u8]) -> usize {
    println!("total bits to parse: {}", bits.len());
    1
}

// fn take_4_bits(input: &[u8]) -> IResult<&[u8], u64> {
//     bits(take::<_, _, _, (_, _)>(11usize))(input)
// }

fn header(i: (&[u8], usize)) -> IResult<(&[u8], usize), Header> {
    let (i, ver): ((&[u8], usize), u8) = take::<_, _, _, _>(3usize)(i)?;
    let (i, type_id): ((&[u8], usize), u8) = take::<_, _, _, _>(3usize)(i)?;
    Ok((i, Header { ver, type_id }))
}

// fn parse_header(i: (&[u8], usize)) -> IResult<(&[u8], usize), Header> {
//     let (i, ver): ((&[u8], usize), u8) = take::<_, _, _, (_, _)>(3usize)(i)?;
//     let (i, type_id): ((&[u8], usize), u8) = take::<_, _, _, (_, _)>(3usize)(i)?;
//     Ok((i, Header{ ver, type_id }))
// }

fn take4(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(4u8)(input)
}

fn take1(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(1u8)(input)
}

fn take11(input: (&[u8], usize)) -> IResult<(&[u8], usize), u16> {
    take(11u8)(input)
}

fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    let mut group = tuple((take1, take4))(input)?;
    let mut words: Vec<u8> = vec![rest(group.1 .1)];
    println!("{:?} {}", group.1, group.0 .1);
    while group.1 .0 == 1 {
        group = tuple((take1, take4))(group.0)?;
        println!("{:?} {}", group.1, group.0 .1);
        words.push(rest(group.1 .1));
    }
    Ok((group.0, combine_literal(&words)))
}

fn combine_literal(words: &[u8]) -> u64 {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| *w as u64 * 2_u64.pow(((words.len() - i - 1) * 4) as u32))
        .sum::<u64>()
}

// fn parse_literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), (u64, usize)> {
//     let mut chunk: ((&[u8], usize), u8) = take::<_, _, _, _>(5usize)(i)?;
//     //println!("{}", chunk.1);
//     let mut done = !leading_bit_on(chunk.1);
//     let mut words: Vec<u8> = vec![rest(chunk.1)];
//     while !done {
//         chunk = take::<_, _, _, _>(5usize)(chunk.0)?;
//         //println!("{}", chunk.1);
//         done = !leading_bit_on(chunk.1);
//         words.push(rest(chunk.1));
//         //println!("parsed: {}  prefix: {}", words.get(0).unwrap(), leading_bit_on(chunk.1));
//         if done {
//             break;
//         }
//     }
//     //println!("words: {:?}", words);
//     let total = words.iter().enumerate().map(|(i, w)| *w as u64 * 2_u64.pow(((words.len() - i - 1) * 4) as u32)).sum();
//     return Ok((chunk.0, (total, words.len()*5)));
// }
//
fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<(Header, u64)>> {
    let (i, a) = take1(input)?;
    let (i, b) = cond(a == 1, take11)(i)?;
    println!("sub packets to read: {}", b.unwrap());
    count(tuple((header, literal)), b.unwrap() as usize)(i)
}

// fn parse_operator(i: (&[u8], usize)) -> IResult<(&[u8], usize), (u64, usize)> {
//     let mut cursor: (&[u8], usize);
//     let (cursor, len_type_id): ((&[u8], usize), u8) = take::<_, _, _, (_, _)>(1usize)(i)?;
//     //let mut cursor: (&[u8], usize) = i;
//     let mut read: usize = 0;
//     let mut versions: u64 = 0;
//     if len_type_id == 1 {
//         let (cursor, sub_packet_count): ((&[u8], usize), usize) = take::<_, _, _, (_, _)>(11usize)(cursor)?;
//         println!("sub_packet_count: {}", sub_packet_count);
//         let mut total_bits_read = 0;
//         for i in 0..sub_packet_count {
//             let (cursor, h)  = parse_header(cursor)?;
//             versions += h.0 as u64;
//             println!("version num: {}", h.0);
//             total_bits_read += 6;
//             if h.1 == 4 {
//                 let (cursor, s1) = parse_literal(cursor)?;
//                 total_bits_read += s1.1;
//                 println!("s: {}  bits read: {}", s1.0, s1.1);
//             } else {
//                 let (cursor, s1) = parse_operator(cursor)?;
//                 versions += s1.0;
//                 total_bits_read += s1.1;
//                 println!("s: {}  bits read: {}", s1.0, s1.1);
//             }
//         }
//         read = 16 + total_bits_read;
//     } else {
//         let (cursor, sub_packets_bits): ((&[u8], usize), usize) = take::<_, _, _, (_, _)>(15usize)(cursor)?;
//         println!("sub_packets_bits: {}", sub_packets_bits);
//         //0011 1000 0000 0000 0110 1111 0100 0101 0010 1001 0001 0010 0000 0000
//         //println!("offset: {}", i.1);
//         let mut total_bits_read = 0;
//         //let mut cursor: (&[u8], usize) = i;
//         while total_bits_read < sub_packets_bits {
//             let (cursor, h)  = parse_header(cursor)?;
//             versions += h.0 as u64;
//             println!("version num: {}", h.0);
//             total_bits_read += 6;
//             if h.1 == 4 {
//                 let (cursor, s1) = parse_literal(cursor)?;
//                 total_bits_read += s1.1;
//                 println!("s: {}  bits read: {}", s1.0, s1.1);
//             } else {
//                 let (cursor, s1) = parse_operator(cursor)?;
//                 versions += s1.0;
//                 total_bits_read += s1.1;
//                 println!("s: {}  bits read: {}", s1.0, s1.1);
//             }
//         }
//         read = 16 + total_bits_read;
//
//
//         //println!("header: {:?}", h);
//         //println!("offset: {}", i.1);
//
//         //println!("offset: {}", i.1);
//         //let (i, h) = parse_header(i)?;
//         //println!("offset: {}", i.1);
//         //println!("header: {:?}", h);
//         //let (i, s2) = parse_literal(i)?;
//         //println!("offset: {}", i.1);
//
//     }
//     Ok((cursor, (versions, read)))
// }

#[allow(dead_code)]
fn leading_bit_on(n: u8) -> bool {
    n & (1 << 4) != 0
}

fn rest(n: u8) -> u8 {
    n & 2_u8.pow(4) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "8A004A801A8002F478";
        let parsed = generator_input(input);
        println!("{:?}", parsed);
        assert_eq!(9, parsed.len());
        // println!("{}", format!("{:b}", parsed.get(1).unwrap()));
    }

    #[test]
    fn test_header() {
        let i = &generator_input("8A004A801A8002F478")[..];
        let (_, hdr) = header((i, 0)).unwrap();
        assert_eq!(4, hdr.ver);
        assert_eq!(2, hdr.type_id);
    }

    #[test]
    fn test_literal() {
        let sample_literal = generator_input("D2FE28");
        let sl = &sample_literal[..];
        let (sl, hdr) = header((sl, 0)).unwrap();
        assert_eq!(6, hdr.ver);
        assert_eq!(4, hdr.type_id);
        let (_, l) = literal(sl).unwrap();
        assert_eq!(2021, l);
        //0101 0010 0010 0100 0000 000
        let test = generator_input("522400");
        let sl = &test[..];
        let (sl, hdr) = header((sl, 0)).unwrap();
        assert_eq!(2, hdr.ver);
        assert_eq!(4, hdr.type_id);
        let (_, l) = literal(sl).unwrap();
        assert_eq!(20, l);
    }

    #[test]
    fn operator_w_length_bits_two_sub_literals() {
        let op_w_two_sub_packets = generator_input("38006F45291200");
        let i = &op_w_two_sub_packets[..];
        let (_, hdr) = header((i, 0)).unwrap();
        assert_eq!(1, hdr.ver);
        assert_eq!(6, hdr.type_id);
        //let (_, l) = parse_operator(i).unwrap();
        //assert_eq!(9, l.0+1);
    }

    #[test]
    fn operator_w_length_packets_three_sub_literals() {
        let op_w_three_sub_packets = generator_input("EE00D40C823060");
        let i = &op_w_three_sub_packets[..];
        let (i, hdr) = header((i, 0)).unwrap();
        assert_eq!(7, hdr.ver);
        assert_eq!(3, hdr.type_id);
        let (_, l) = operator(i).unwrap();
        println!("result {:?}", l);
    }
}
