use regex::Regex;
use std::{collections::{BTreeMap}, vec};

struct CargoCrane {
    stacks: BTreeMap<u8, Vec<String>>,
}

impl CargoCrane {
    fn new() -> Self {
        CargoCrane {
            stacks: BTreeMap::new(),
        }
    }
    fn set_stacks(&mut self, stacks: BTreeMap<u8, Vec<String>>) {
        self.stacks = stacks;
    }

    fn move_crate(&mut self, crates: u8, from: u8, to: u8) {
        let len_from = self.stacks.get(&from).unwrap().len();
    
        let mut crates_from: Vec<String> = self.stacks.get_mut(&from).unwrap()
        .drain(len_from-usize::from(crates)..).collect();

        self.stacks
            .get_mut(&to)
            .unwrap()
            .extend(crates_from);
      }
}

fn main() {
    let input: Vec<&str> = include_str!("../input/input.txt").split("\n").collect();
    let mut stacks: BTreeMap<u8, Vec<String>> = BTreeMap::new();
    let regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    let mut crane: CargoCrane = CargoCrane::new();
    for line in input {
        match regex.captures(line) {
            Some(caps) => {
                let crate_id: u8 = caps.get(1).unwrap().as_str().parse().unwrap();
                let from = caps.get(2).unwrap().as_str().parse().unwrap();
                let to = caps.get(3).unwrap().as_str().parse().unwrap();

                crane.move_crate(crate_id, from, to)
            }
            None => {
                if !line.is_empty() || !line.contains("1") {
                    line.char_indices().filter(|f| f.0 % 4 == 0).for_each(|f| {
                        if !f.1.is_whitespace() {
                            match stacks.get_mut(((f.0 + 4) / 4).try_into().as_ref().unwrap()) {
                                Some(x) => x.insert(0, line[f.0..f.0 + 3].to_string()),
                                None => {
                                    let mut vecstr = vec![];
                                    vecstr.insert(0, line[f.0..f.0 + 2].to_string());
                                    stacks.insert(((f.0 + 4) / 4).try_into().unwrap(), vecstr);
                                }
                            }
                        }
                    });
                    if stacks.len() == 9 {
                        //Build CargoCrane
                        crane.set_stacks(stacks.clone());
                    }
                }
            }
        }
    }

    //top_crates  
    for mut stack in crane.stacks {
        println!("{:?}", stack.1.pop().unwrap());
    }
}
