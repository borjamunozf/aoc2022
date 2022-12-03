use std::{
    cmp::Reverse,
    fs::{self, File},
    io::{self, BufRead},
    num::ParseIntError,
    path::Path,
};

fn main() {
    let mut elfs = Vec::new();
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut cals = 0;
        for line in lines {
            if let Ok(freq) = line {
                let cal_lin: Result<u32, ParseIntError> = freq.parse();
                if cal_lin.is_err() {
                    add_elf_cal(&mut elfs, cals);
                    cals = 0;
                } else {
                    cals = cal_lin.unwrap() + cals;
                }
            }
        }

        println!("Max value is: {}", elfs.iter().max().unwrap());
        println!("Total freq: {}", cals);

        //part 2
        //vec keys must implement Copy trait to avoid lifetime
        elfs.sort_by_key(|c| Reverse(*c));

        elfs.truncate(3);
        let top3: u32 = elfs.iter().sum();
        println!("Total cals: 3 top elf -> {}", top3);
    }

    //read to memory, apply map & sum.
}

fn add_elf_cal(list: &mut Vec<u32>, cals: u32) -> &mut Vec<u32> {
    list.push(cals);
    return list;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
