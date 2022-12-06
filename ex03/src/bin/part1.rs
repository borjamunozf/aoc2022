use std::collections::HashSet;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

//pt 1 ->
//case sensitive
//rucksack <-> 2 compartments. One item type on one of the 2 compartments.
//half nº1 - nº1 compartment // half 2 - nº compartment
fn main() {
    if let Ok(lines) = read_lines("./input/input_1.txt") {
        let mut sum_rucksacks = 0;
        for line in lines {
            if let Ok(ruckstr) = line {
                let (first_comp, second_comp) = ruckstr.split_at(ruckstr.len() / 2);
                let mut first_set = HashSet::new();
                let mut second_set = HashSet::new();

                println!("Rucksack: {}", ruckstr);
                println!("First comp: {}", first_comp);
                println!("Second comp: {}", second_comp);

                for c in first_comp.chars() {
                    first_set.insert(c);
                }
                for c in second_comp.chars() {
                    second_set.insert(c);
                }

                let rucksack_sum: i32 = first_set
                    .intersection(&second_set)
                    .into_iter()
                    .map(|&pr| {
                        match pr {
                            'A'..='Z' => 'Z' as i32 - 'A' as i32 + pr as i32 - 'A' as i32 + 2, //65 - 90 ascii A-Z zz
                            'a'..='z' => pr as i32 - 'a' as i32 + 1,
                            _ => 0,
                        }
                    })
                    .sum();

                println!("Rucksack sum line: {}", rucksack_sum);
                sum_rucksacks = rucksack_sum + sum_rucksacks;
                println!("Sum sacks: {}", sum_rucksacks);
            }
        }
    }
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
