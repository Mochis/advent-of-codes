use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BTreeMap;

fn main() {
    let elves_cals = lines("../input");
    let mut total_cals = 0;
    let mut elf_ranking = BTreeMap::new();

    for elf_cal in elves_cals {
        let vari = elf_cal.unwrap();
        let partial = vari.as_str();
        match partial {
            "" => {
                match elf_ranking.get_key_value(&total_cals) {
                    None => {
                        elf_ranking.insert(total_cals, 1);
                    },
                    Some((_, v)) => {
                        elf_ranking.insert(total_cals, v + 1);
                    },
                };
                total_cals = 0;
            },
            cals => {
                total_cals += cals.parse::<i32>().unwrap();
            },
        };
    }

    let mut sum = 0;
    let mut it = elf_ranking.iter().rev();
    for i in 0..3 {
        match it.next() {
            Some((&k, &v)) => {
                sum += k * v
            },
            None => panic!("Error"),
        }
        
    }
    println!("The sum is: {}", sum);
}

fn lines(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(path)
        .expect("This program expects the file being included in the project");
    io::BufReader::new(file).lines()
}