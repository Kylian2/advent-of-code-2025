use std::{cmp::max, fs::File, io::Read, thread::current};

fn read_input() -> String {
    let file_r = File::open("input.txt");
    let mut content = String::new();
    if let Ok(mut file) = file_r {
        file.read_to_string(&mut content);
    } else if let Err(error) = file_r {
        panic!("Erreur : {}", error);
    }
    String::from(content.trim())
}

fn part_1() {
    let content = read_input();

    let mut total_capacity = 0;
    for bank in content.lines() {
        let mut bank_capacities = bank.chars().rev().map(|c| c.to_digit(10).unwrap());

        // Initialisation
        let mut current_switch_on = (0, bank_capacities.next().unwrap());

        for capacity in bank_capacities {
            // On fait les ajustements (si plus grande capacité on remplace la première cellule, et on décale si c'est rentable)
            if current_switch_on.0 <= capacity {
                if current_switch_on.1 < current_switch_on.0 {
                    current_switch_on.1 = current_switch_on.0;
                }
                current_switch_on.0 = capacity;
            }
        }
        total_capacity += current_switch_on.0 * 10 + current_switch_on.1;
    }
    println!("Total capacity = {total_capacity}");
}

fn part_2() {
    let content = read_input();

    let mut total_capacity = 0;
    for bank in content.lines() {
        let mut bank_capacities = bank.chars().rev().map(|c| c.to_digit(10).unwrap() as u128);
        let mut current_switch_on: [u128; 12] = [0; 12];

        // Initialisation de 12 premiers choix
        for i in 0..12 {
            current_switch_on[11 - i] = bank_capacities.next().unwrap();
        }

        for capacity in bank_capacities {
            // Si on trouve un capacité plus grande, on la mets en tête
            if current_switch_on[0] <= capacity {
                let mut temp = current_switch_on[0];
                current_switch_on[0] = capacity;

                // On parcours la liste en decallant jusqu'a ce que ce ne soit plus rentable de décaller
                // (ie le (n-1)-ième est plus grand ou égal au n-ième élément des 12)
                for i in 1..current_switch_on.len() {
                    if temp >= current_switch_on[i] {
                        let temp_2 = current_switch_on[i];
                        current_switch_on[i] = temp;
                        temp = temp_2;
                    } else {
                        break;
                    }
                }
            }
        }

        // On calcul la valeur de la bank
        for (i, val) in current_switch_on.into_iter().rev().enumerate() {
            total_capacity += val * u128::pow(10, i as u32);
        }
    }
    println!("Total capacity = {total_capacity}");
}

fn main() {
    part_2();
}
