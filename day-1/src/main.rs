use std::{
    cmp::min,
    fs::{File, ReadDir},
    io::Read,
    u32,
};

fn part_1() {
    let file_r = File::open("input.txt");
    let mut content = String::new();
    if let Ok(mut file) = file_r {
        file.read_to_string(&mut content);
    } else if let Err(error) = file_r {
        panic!("Erreur : {}", error);
    }

    let mut dial: i32 = 50;
    let mut passcode = 0;

    for i in content.lines() {
        let side = i.get(0..=0);
        let val = (i.get(1..).unwrap()).parse::<i32>().unwrap();

        if let Some(side) = side {
            match side {
                "L" => dial = (dial - val).rem_euclid(100),

                "R" => dial = (dial + val).rem_euclid(100),
                _ => panic!("Coté non reconnu"),
            }
        }

        if dial == 0 {
            passcode += 1;
        }
    }

    println!("Le mot de passe est {passcode}");
}

fn part_2() {
    let file_r = File::open("input.txt");
    let mut content = String::new();
    if let Ok(mut file) = file_r {
        file.read_to_string(&mut content);
    } else if let Err(error) = file_r {
        panic!("Erreur : {}", error);
    }

    let mut dial: i32 = 50;
    let mut passcode = 0;

    for i in content.lines() {
        let side = i.get(0..=0);
        let val = (i.get(1..).unwrap()).parse::<i32>().unwrap();

        if let Some(side) = side {
            match side {
                "L" => {
                    for i in 0..val {
                        dial -= 1;
                        if dial < 0 {
                            dial = dial + 100;
                        }
                        if dial == 0 {
                            passcode += 1
                        }
                    }
                }

                "R" => {
                    for i in 0..val {
                        dial += 1;
                        if dial >= 100 {
                            dial = dial - 100;
                        }
                        if dial == 0 {
                            passcode += 1
                        }
                    }
                }
                _ => panic!("Coté non reconnu"),
            }
        }
    }

    println!("Le mot de passe est {passcode}");
}

fn main() {
    part_2();
}
