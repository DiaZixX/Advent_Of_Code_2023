use std::{time::Instant, collections::HashMap};

fn main() {
    let ex_input: &str = include_str!("ex_input.txt");
    let input: &str = include_str!("input.txt");
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: u64 = part1(ex_input);

    let st1 = Instant::now();
    let output1: u64 = part1(input);
    let et1 = Instant::now();
    let rt1 = et1 - st1;
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: u64 = part2(ex_input);

    let st2 = Instant::now();
    let output2: u64 = part2(input);
    let et2 = Instant::now();
    let rt2 = et2 - st2;

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 1320 et est : {}", ex_output1);
    println!("L'output 1 est : {} [Exécuté en : {:?}]", output1, rt1);  
    println!("L'output de l'exemple 2 doit être 145 et est : {}", ex_output2);
    println!("L'output 2 est : {} [Exécuté en : {:?}]", output2, rt2);  
}

fn part1(texte: &str) -> u64 {
    let mut score : u64 = 0;

    let lines = texte.lines();

    for line in lines {
        let mots = line.split(',');
        for mot in mots {
            let caracters : Vec<char> = mot.chars().collect();
            let mut current_value : u64 = 0;
            for carac in caracters {
                current_value = apply_hash(carac, current_value);
            }  
            // println!("{}", current_value);
            score += current_value; 
        }
    }

    return score
}

fn apply_hash(c: char, current_value: u64) -> u64 {
    let code_ascii = c as u8;
    let ret: u64 = ((current_value + code_ascii as u64) * 17) % 256;
    return ret
}

fn part2(texte: &str) -> u64 {
    let mut score : u64 = 0;

    let lines = texte.lines();

    let mut boxes : Vec<Vec<&str>> = vec![vec!["";1]; 256];
    let mut dict : HashMap<&str, u8> = HashMap::new();

    for line in lines {
        let mots = line.split(',');

        for mot in mots {
            let caracters : Vec<char> = mot.chars().collect();
            for i in 0..caracters.len() {
                if caracters[i] == '-' {
                    remove_from_boxes(&mut boxes, &mot[0..i])
                }
                if caracters[i] == '=' {
                    let is_in = is_in_boxes(&boxes, &mot[0..i]);
                    if !is_in {
                        let mut index = 0;
                        for k in 0..i {
                            index = apply_hash(caracters[k], index);
                        }
                        if boxes[index as usize].is_empty() {
                        }
                        else if boxes[index as usize][0].eq("") {
                            boxes[index as usize].pop();
                        }
                        boxes[index as usize].push(&mot[0..i]);
                    }
                    let value = match caracters[i+1].to_digit(10) {
                        Some(x) => x,
                        None => 0,
                    };
                    dict.insert(&mot[0..i], value as u8);
                }
            }

        }
    }

    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let value = match dict.get(boxes[i][j]) {
                Some(x) => *x,
                None => continue
            };
            score += (i as u64+1) * (j as u64+1) * value as u64;
        }
    }

    return score
}

fn remove_from_boxes(mat: &mut Vec<Vec<&str>>, seq: &str) -> () {
    for i in 0..mat.len() {
        if mat[i].contains(&seq) {
            mat[i].retain(|&x| !x.eq(seq));
            break
        }
    }
}

fn is_in_boxes(mat: &Vec<Vec<&str>>, seq: &str) -> bool {
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j].eq(seq) {
                return true;
            }
        }
    }
    return false
}
