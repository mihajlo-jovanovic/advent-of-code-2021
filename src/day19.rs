use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::{enumerate, Itertools};
use nalgebra::{distance, Dynamic, Matrix3, MatrixXx3, OMatrix, Point3, RowVector3, U3};

#[aoc_generator(day19)]
fn generator_input(input: &str) -> Vec<Vec<(i16, i16, i16)>> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|s| {
                    let mut coords = s.split(',');
                    (
                        coords.next().unwrap().parse::<i16>().unwrap(),
                        coords.next().unwrap().parse::<i16>().unwrap(),
                        coords.next().unwrap().parse::<i16>().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

#[aoc(day19, part1)]
fn part1(scanner_reports: &[Vec<(i16, i16, i16)>]) -> usize {
    let (beacons, _) = assemble_full_map(scanner_reports);
    beacons.iter().fold(HashSet::new(), |x, y| x.union(&HashSet::from_iter(y.iter())).cloned().collect()).len()
}

#[aoc(day19, part2)]
fn part2(scanner_reports: &[Vec<(i16, i16, i16)>]) -> i16 {
    let (_, scanners) = assemble_full_map(scanner_reports);
    scanners.iter().combinations(2).map(|coll| manhattan(coll[0], coll[1])).max().unwrap()
}

fn assemble_full_map(scanner_reports: &[Vec<Position>]) -> (Vec<Vec<Position>>, Vec<Position>) {
    let mut beacons = vec![];
    let mut scanners = vec![(0, 0, 0)];
    for i in scanner_reports {
        beacons.push(i.clone());
    }
    let overlap = overlap(scanner_reports);
    let mut found: Vec<usize> = vec![0];
    while found.len() < scanner_reports.len() {
        for (s1, s2) in &overlap {
            if found.contains(s1) && !found.contains(s2) {
                println!("Joining {} and {}", s1, s2);
                let tmp = &beacons[*s1].clone();
                scanners.push(align(tmp, &mut beacons[*s2]));
                found.push(*s2);
            }
            if !found.contains(s1) && found.contains(s2) {
                println!("Joining {} and {}", s2, s1);
                let tmp = &beacons[*s2].clone();
                scanners.push(align(tmp, &mut beacons[*s1]));
                found.push(*s1)
            }
        }
    }
    (beacons, scanners)
}

fn manhattan(p1: &Position, p2: &Position) -> i16 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

fn overlap(scanners: &[Vec<(i16, i16, i16)>]) -> Vec<(usize, usize)> {
    let similarity: Vec<usize> = scanners
        .iter()
        .combinations(2)
        .map(|s| {
            let d1 = distances(s[0]);
            let d2 = distances(s[1]);
            let s1_dist = d1
                .iter()
                .map(|f| (f * 10000.0).round() as u32);
            let s2_dist = d2
                .iter()
                .map(|f| (f * 10000.0).round() as u32);
            let set1: HashSet<u32> = HashSet::from_iter(s1_dist);
            let set2 = HashSet::from_iter(s2_dist);
            set1.intersection(&set2).count()
        })
        .collect();
    //println!("{:?}", similarity);
    let sequence = 0..scanners.len();
    sequence.combinations(2).enumerate().filter(|(i, _)| similarity[*i] >= 66).map(|(_, el)| (el[0], el[1])).collect()
}

type Position = (i16, i16, i16);

fn aligned(m1: &OMatrix<i16, Dynamic, U3>, m2: &OMatrix<i16, Dynamic, U3>) -> bool {
    let x = m1[(0, 0)] - m2[(0, 0)];
    let y = m1[(0, 1)] - m2[(0, 1)];
    let z = m1[(0, 2)] - m2[(0, 2)];
    for i in 1..12 {
        if m1[(i, 0)] - m2[(i, 0)] != x {
            return false;
        }
        if m1[(i, 1)] - m2[(i, 1)] != y {
            return false;
        }
        if m1[(i, 2)] - m2[(i, 2)] != z {
            return false;
        }
    }
    true
}

fn align(s1: &[Position], s2: &mut [Position]) -> Position {
    let matching = beacons(s1, s2);
    if matching.len() < 12 {
        panic!("not overlapping");
    }
    let rows: Vec<RowVector3<i16>> = matching.iter().map(|x| RowVector3::new(s1[x.0].0, s1[x.0].1, s1[x.0].2)).collect();
    let a = MatrixXx3::from_rows(&rows);
    let rows2: Vec<RowVector3<i16>> = matching.iter().map(|x| RowVector3::new(s2[x.1].0, s2[x.1].1, s2[x.1].2)).collect();
    let b = MatrixXx3::from_rows(&rows2);
    let special = special();
    for m in &special {
        let res = b.clone() * m;
        if aligned(&res, &a) {
            //println!("found it: {:?}", m);
            let offset: RowVector3<i16> = RowVector3::new(a[(0, 0)] - res[(0, 0)], a[(0, 1)] - res[(0, 1)], a[(0, 2)] - res[(0, 2)]);
            //println!("{:?}", offset);
            // align scanner b
            let rw: Vec<RowVector3<i16>> = std::iter::repeat(offset).take(s2.len()).into_iter().collect();
            let offset_as_matrix = MatrixXx3::from_rows(&rw);
            let rw2: Vec<RowVector3<i16>> = s2.iter().map(|p| RowVector3::new(p.0, p.1, p.2)).collect();
            let s2_as_matrix = MatrixXx3::from_rows(&rw2);
            let align = s2_as_matrix * m + offset_as_matrix;
            //align += offset_as_matrix;
            let new_scanner: Vec<Position> = align.row_iter().map(|r| (r[(0, 0)], r[(0, 1)], r[(0, 2)])).collect();
            for (i, pos) in enumerate(new_scanner) {
                s2[i] = pos;
            }
            return (a[(0, 0)] - res[(0, 0)], a[(0, 1)] - res[(0, 1)], a[(0, 2)] - res[(0, 2)]);
        }
    }
    panic!("not found")
}

fn determinant(m: &Matrix3<i16>) -> i16 {
    m[(0, 0)] * (m[(1, 1)] * m[(2, 2)] - (m[(1, 2)] * m[(2, 1)])) -
        m[(0, 1)] * (m[(1, 0)] * m[(2, 2)] - (m[(1, 2)] * m[(2, 0)])) +
        m[(0, 2)] * (m[(1, 0)] * m[(2, 1)] - (m[(1, 1)] * m[(2, 0)]))
}

fn special() -> Vec<Matrix3<i16>> {
    let a = [-1, 0, 1];
    itertools::iproduct!(&a, &a, &a, &a, &a, &a, &a, &a, &a).
        map(|(a, b, c, d, e, f, g, h, i)|
            Matrix3::new(*a, *b, *c, *d, *e, *f, *g, *h, *i)).
        filter(|m| determinant(m) == 1).collect()
}

fn distances(col: &[(i16, i16, i16)]) -> Vec<f64> {
    col.iter()
        .map(|p| Point3::new(p.0 as f64, p.1 as f64, p.2 as f64))
        .combinations(2)
        .map(|s| distance(&s[0], &s[1]))
        .collect()
}

fn beacons(s1: &[Position], s2: &[Position]) -> Vec<(usize, usize)> {
    let mut matching = vec![];
    for (i, pos) in enumerate(s1) {
        let set1 = dist(s1, pos);
        for (j, pos) in enumerate(s2) {
            let set2 = dist(s2, pos);
            if set1.intersection(&set2).count() >= 12 {
                matching.push((i, j));
            }
        }
    }
    matching
}

fn dist(s1: &[Position], pos: &Position) -> HashSet<u32> {
    HashSet::from_iter(s1.iter()
        .map(|(x, y, z)| Point3::new(*x as f64, *y as f64, *z as f64))
        .map(|p| (distance(&Point3::new(pos.0 as f64, pos.1 as f64, pos.2 as f64), &p) * 10000.0).round() as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_parsing() {
        let parsed = generator_input(TEST_INPUT);
        assert_eq!(5, parsed.len());
        assert_eq!(25, parsed[0].len());
        assert_eq!(404, parsed[0][0].0);
        assert_eq!(-14, parsed[4][25].2);
        assert_eq!(1317, distances(&parsed[0]).first().unwrap().round() as u16);
    }

    #[test]
    fn test_overlap() {
        let overlap = overlap(&generator_input(TEST_INPUT));
        assert_eq!(4, overlap.len());
        assert_eq!((0, 1), overlap[0]);
        assert!(overlap.contains(&(1, 4)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(79, part1(&generator_input(TEST_INPUT)));
    }

    #[test]
    fn test_beacons() {
        let parsed = generator_input(TEST_INPUT);
        let matches = beacons(&parsed[0], &parsed[1]);
        println!("{:?}", matches);
        assert_eq!(12, matches.len());
        assert!(matches.contains(&(0, 3)));
    }

    #[test]
    fn test_align() {
        let parsed = generator_input(TEST_INPUT);
        let mut scanners = parsed.clone();
        //let mut s2 = scanners[1].clone();
        let matches = align(&parsed[0], &mut scanners[1]);
        println!("{:?}", matches);
        println!("{:?}", scanners[1]);
        let beacons: HashSet<&(i16, i16, i16)> = HashSet::from_iter(scanners[0].iter());
        let beacons2: HashSet<&(i16, i16, i16)> = HashSet::from_iter(scanners[1].iter());
        println!("{}", beacons.union(&beacons2).count());
    }
}