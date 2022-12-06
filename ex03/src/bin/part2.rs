use std::collections::HashSet;
use std::hash::Hash;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

struct Rucksack {
    items: String,
}

/*impl Rucksack {
    fn new(items: &str) -> Rucksack {
        /*  Rucksack {
            items: String::new(items),
        }*/
        Rucksack { items: () }
    }
} */
//pt 1 ->
//case sensitive
//rucksack <-> 2 compartments. One item type on one of the 2 compartments.
//half nº1 - nº1 compartment // half 2 - nº compartment

fn main() {
    let rucksacks: Vec<&str> = include_str!("../../input/input_1.txt").split("\n").collect();

    let mut ruckgroups = vec![];
    let mut total_badges = 0;
    for group in rucksacks.chunks(3) {
        let mut set = HashSet::new();
        let mut set2 = HashSet::new();
        let mut set3 = HashSet::new();
        for c in group[0].chars() {
            set.insert(c);
        }

        ruckgroups.push(set);

        for c in group[1].chars() {
            set2.insert(c);
        }

        ruckgroups.push(set2);

        for c in group[2].chars() {
            set3.insert(c);
        }
        ruckgroups.push(set3);

        let badge_group: i32 = ruckgroups.iter()
        .fold(ruckgroups[0].clone(), |acc, r| {
            acc.intersection(r).cloned().collect()
        })
        .iter()
        .map(|m| match m {
                'A'..='Z' => 'Z' as i32 - 'A' as i32 + *m as i32 - 'A' as i32 + 2, //65 - 90 ascii A-Z zz
                'a'..='z' => *m as i32 - 'a' as i32 + 1,
                _ => 0,
            }
        ).sum();

        ruckgroups.clear();
        total_badges = total_badges + badge_group;
    }

    println!("common item type: {:?}", total_badges);
}
