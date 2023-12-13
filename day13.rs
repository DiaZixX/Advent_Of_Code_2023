use std::time::Instant;

fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    
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

    println!("L'output de l'exemple 1 doit être 405 et est : {}", ex_output1);
    println!("L'output 1 est : {} [Exécuté en : {:?}]", output1, rt1);  
    println!("L'output de l'exemple 2 doit être 400 et est : {}", ex_output2);
    println!("L'output 2 est : {} [Exécuté en : {:?}]", output2, rt2);  
}

fn part1(texte : &str) -> u64 {
    let mut score: u64 = 0;
    let cases = texte.split("\n\n");

    for case in cases {
        let mut mat : Vec<Vec<char>> = Vec::new();
        let lines = case.split("\n");
        for line in lines {
            let temp: Vec<char> = line.chars().collect();
            if !temp.is_empty() {
                mat.push(temp);
            }
        }

        for i in 0..mat.len()-1 {
            if test_hori_sym(&mat, i, i+1){
                let temp_score = (i+1) * 100;
                score += temp_score as u64;
                break;
            }
        }
        for i in 0..mat[0].len()-1 {
            if test_vert_sym(&mat, i, i+1) {
                let temp_score = i+1;
                score += temp_score as u64;
                break;
            }
        }

    }
    return score;
}

fn test_vert_sym(mat : &Vec<Vec<char>>, n: usize, m: usize) -> bool {
    let distance_to_check = min(n+1, mat[0].len() - m);
    for i in 0..distance_to_check {
        for j in 0..mat.len() {
            if mat[j][n-i] != mat[j][m+i] {
                return false
            }
        }
    } 
    return true
}

fn test_hori_sym(mat : &Vec<Vec<char>>, n: usize, m: usize) -> bool  {
    let distance_to_check = min(n+1, mat.len() - m);
    for i in 0..distance_to_check {
        for j in 0..mat[0].len() {
            if mat[n-i][j] != mat[m+i][j] {
                return false
            }
        }
    }
    return true 
}

fn part2(texte : &str) -> u64 {
    let mut score: u64 = 0;
    let cases = texte.split("\n\n");

    for case in cases {
        let mut mat : Vec<Vec<char>> = Vec::new();
        let lines = case.split("\n");
        for line in lines {
            let temp: Vec<char> = line.chars().collect();
            if !temp.is_empty() {
                mat.push(temp);
            }
        }

        for i in 0..mat.len()-1 {
            if test_hori_sym2(&mat, i, i+1){
                let temp_score = (i+1) * 100;
                score += temp_score as u64;
                break;
            }
        }
        for i in 0..mat[0].len()-1 {
            if test_vert_sym2(&mat, i, i+1) {
                let temp_score = i+1;
                score += temp_score as u64;
                break;
            }
        }

    }
    return score;
}

fn test_vert_sym2(mat : &Vec<Vec<char>>, n: usize, m: usize) -> bool {
    let distance_to_check = min(n+1, mat[0].len() - m);
    let mut one_mistake = false;
    for i in 0..distance_to_check {
        for j in 0..mat.len() {
            if mat[j][n-i] != mat[j][m+i] {
                if !one_mistake {
                    one_mistake = true;
                }
                else {
                    return false
                }
            }
        }
    } 
    if !one_mistake {
        return false;
    }
    else {
        return true 
    }
}

fn test_hori_sym2(mat : &Vec<Vec<char>>, n: usize, m: usize) -> bool  {
    let distance_to_check = min(n+1, mat.len() - m);
    let mut one_mistake = false;
    for i in 0..distance_to_check {
        for j in 0..mat[0].len() {
            if mat[n-i][j] != mat[m+i][j] {
                if !one_mistake {
                    one_mistake = true;
                }
                else {
                    return false
                }
            }
        }
    }
    if !one_mistake {
        return false;
    }
    else {
        return true 
    }
}

fn min(a: usize, b: usize) -> usize{
    if a < b {
        return a
    }
    else {
        return b 
    }
}
