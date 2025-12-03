use fancy_regex::Regex;
use std::{fs::File, io::Read};

fn part_1() {
    let file_r = File::open("input.test");
    let mut content = String::new();
    if let Ok(mut file) = file_r {
        file.read_to_string(&mut content);
    } else if let Err(error) = file_r {
        panic!("Erreur : {}", error);
    }
    content = String::from(content.trim());

    let mut total: u128 = 0;

    for range in content.split(",") {
        let (start_str, end_str) = range.split_once("-").unwrap();
        let start = (start_str).parse::<u128>().unwrap();
        let end = (end_str).parse::<u128>().unwrap();
        for i in start..=end {
            let id = i.to_string();
            if id.len() % 2 == 0 {
                let parts = id.split_at(id.len() / 2);
                if parts.0 == parts.1 {
                    total += i;
                }
            }
        }
    }
    println!("{total}");
}

fn part_2() {
    let file_r = File::open("input.txt");
    let mut content = String::new();
    if let Ok(mut file) = file_r {
        file.read_to_string(&mut content);
    } else if let Err(error) = file_r {
        panic!("Erreur : {}", error);
    }
    content = String::from(content.trim());

    let mut total: u128 = 0;

    for range in content.split(",") {
        let (start_str, end_str) = range.split_once("-").unwrap();
        let start = (start_str).parse::<u128>().unwrap();
        let end = (end_str).parse::<u128>().unwrap();
        println!("{start} - {end}");
        for i in start..=end {
            let id = i.to_string();

            let re = Regex::new(r"^(?<pattern>[0-9]*)\k<pattern>{1,}$").unwrap();
            if let Ok(res) = re.is_match(&id)
                && res
            {
                total += i;
            }
        }
    }
    println!("{total}");
}

fn main() {
    part_2();
}
