use std::collections::BTreeMap;
use itertools::enumerate;

#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<u8> {
    input.lines().next().unwrap().split(',').map(|x| x.parse().unwrap()).collect()
}
// todo: get this working; for now it was easier to do it in Clojure, but would like to use this to
// understand a bit more about how Rust works `under the hood`
#[allow(dead_code)]
fn seq(l: Vec<u8>, mut days: u8) -> usize {
    let mut lanternfish = BTreeMap::new();
    for i in l {
        *lanternfish.entry(i).or_insert(0) += 1;
    }
    println!("{:?}", lanternfish);
    let mut min = lanternfish.keys().next().cloned().unwrap();
    while days > min {
        //let new_species: usize = 0;
        //lanternfish = lanternfish.iter_mut().map(|(k,v) | (k,*v-min)).collect();
        // for (k,v) in &lanternfish {
        //     if *k == min {
        //         lanternfish.remove(&k);
        //         new_species = *v;
        //     } else {
        //         lanternfish.insert(*k, v-min as usize);
        //     }
        //     lanternfish.insert(8, new_species);
        days -= min + 1;
        min = lanternfish.keys().next().cloned().unwrap();
        println!("days: {} min: {}  fish: {:?}", days, min, lanternfish);
    }
    lanternfish.len()
}

#[aoc(day6, part1)]
fn p1(state: &[u8]) -> usize {
    let mut s: Vec<u8> = state.to_vec();
    let mut count: usize = 0;

    for _ in 0..80 {
        let mut new_state: Vec<u8> = vec![];
        for (j, l) in enumerate(&mut s) {
            if *l == 0 {
                new_state.insert(j, 6);
                new_state.push(8);
            } else {
                new_state.insert(j, *l - 1);
            }
        }
        s = new_state;
        //println!("{:?}", s);
        count = s.len();
        print! {"{},", count};
    }
    count
}

#[test]
fn test_p1() {
    let i = generator_input("3,4,3,1,2");
    assert_eq!(5934, p1(&i));
}

#[test]
fn test_seq() {
    assert_eq!(1, seq(generator_input("3,4,3,1,2"), 3));
    // assert_eq!(2, seq(generator_input("3,4,3,1,2"), 4));
    // assert_eq!(3, seq(generator_input("3,4,3,1,2"), 11));
    // assert_eq!(4, seq(generator_input("3,4,3,1,2"), 13));
}