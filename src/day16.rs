use nom::multi::count;
use nom::sequence::tuple;
use nom::{bits::complete::take, IResult};

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
fn part1(bits: &[u8]) -> u64 {
    //println!("total bits to parse: {}", bits.len());
    packet((bits, 0)).unwrap().1
}

fn header(i: (&[u8], usize)) -> IResult<(&[u8], usize), Header> {
    let (i, ver): ((&[u8], usize), u8) = take(3usize)(i)?;
    let (i, type_id): ((&[u8], usize), u8) = take(3usize)(i)?;
    Ok((i, Header { ver, type_id }))
}

fn take4(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(4u8)(input)
}

fn take1(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(1u8)(input)
}

fn take11(input: (&[u8], usize)) -> IResult<(&[u8], usize), u16> {
    take(11u8)(input)
}

fn take15(input: (&[u8], usize)) -> IResult<(&[u8], usize), u16> {
    take(15u8)(input)
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    let (i, h) = header(input)?;
    if h.type_id == 4 {
        let (i, v) = literal(i)?;
        Ok((i, v))
    } else {
        let (i, v) = operator(i, h.type_id)?;
        Ok((i, v))
    }
}

fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    let mut group = tuple((take1, take4))(input)?;
    let mut words: Vec<u8> = vec![rest(group.1 .1)];
    //println!("{:?} {}", group.1, group.0 .1);
    while group.1 .0 == 1 {
        group = tuple((take1, take4))(group.0)?;
        //println!("{:?} {}", group.1, group.0 .1);
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

fn operator(input: (&[u8], usize), type_id: u8) -> IResult<(&[u8], usize), u64> {
    let (i, a) = take1(input)?;
    if a == 0 {
        let (i, a) = take15(i)?;
        //println!("bits to read: {}", a);
        parse_by_packet_len(i, a, type_id)
    } else {
        let (i, a) = take11(i)?;
        //println!("sub packets to read: {}", a);
        let (i, col) = count(packet, a as usize)(i)?;
        //let ver_sum = col.iter().sum();
        //println!("ver_sum: {}", ver_sum);
        Ok((i, reduce_by_type_id(col, type_id)))
    }
}

fn reduce_by_type_id(col: Vec<u64>, type_id: u8) -> u64 {
    match type_id {
        0 => col.iter().sum(),
        1 => col.iter().product(),
        2 => *col.iter().min().unwrap(),
        3 => *col.iter().max().unwrap(),
        5 => {
            if col[0] > col[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if col[0] < col[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if col[0] == col[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid type ID: {}", type_id),
    }
}

fn parse_by_packet_len(
    input: (&[u8], usize),
    len_bits: u16,
    type_id: u8,
) -> IResult<(&[u8], usize), u64> {
    let mut total_bits_read = 0;
    let mut i = input;
    let mut col: Vec<u64> = vec![];
    let start_len = input.0.len();
    let start_offset = input.1;
    while total_bits_read < len_bits {
        //println!("len (byte): {} offset: {}", input.0.len(), input.1);
        let (i2, v) = packet(i)?;
        i = i2;
        //println!("{:?}", v);
        //println!("len (byte): {} offset: {}", i.0.len(), i.1);

        total_bits_read =
            (((start_len - i.0.len()) * 8) as i16 + (i.1 as i16 - start_offset as i16)) as u16;
        //println!("total read to far: {}", total_bits_read);

        col.push(v);
    }
    Ok((i, reduce_by_type_id(col, type_id)))
}

fn rest(n: u8) -> u8 {
    n & (2_u8.pow(4) - 1)
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
        let (i, hdr) = header((i, 0)).unwrap();
        assert_eq!(1, hdr.ver);
        assert_eq!(6, hdr.type_id);
        let (_, l) = operator(i, hdr.type_id).unwrap();
        println!("result {:?}", l);
        assert_eq!(8, l);
    }

    #[test]
    fn operator_w_length_packets_three_sub_literals() {
        let op_w_three_sub_packets = generator_input("EE00D40C823060");
        let i = &op_w_three_sub_packets[..];
        let (i, hdr) = header((i, 0)).unwrap();
        assert_eq!(7, hdr.ver);
        assert_eq!(3, hdr.type_id);
        let (_, l) = operator(i, hdr.type_id).unwrap();
        println!("result {:?}", l);
        assert_eq!(7, l);
    }

    #[test]
    fn operators_nested() {
        let bits = generator_input("8A004A801A8002F478");
        let i = &bits[..];
        let (i, hdr) = header((i, 0)).unwrap();
        assert_eq!(4, hdr.ver);
        assert_eq!(2, hdr.type_id);
        let (_, l) = operator(i, hdr.type_id).unwrap();
        println!("result {:?}", l);
        assert_eq!(12, l);
        assert_eq!(16, part1(&bits));
    }

    #[test]
    fn test_part1() {
        // assert_eq!(12, part1(&generator_input("620080001611562C8802118E34")));
        // assert_eq!(23, part1(&generator_input("C0015000016115A2E0802F182340")));
        // assert_eq!(
        //     31,
        //     part1(&generator_input("A0016C880162017C3686B18A3D4780"))
        // );
        assert_eq!(3, part1(&generator_input("C200B40A82")));
        assert_eq!(54, part1(&generator_input("04005AC33890")));
        assert_eq!(7, part1(&generator_input("880086C3E88112")));
        assert_eq!(1, part1(&generator_input("9C0141080250320F1802104A08")));
    }
}
