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

    println!("L'output de l'exemple 1 doit être 136 et est : {}", ex_output1);
    println!("L'output 1 est : {} [Exécuté en : {:?}]", output1, rt1);  
    println!("L'output de l'exemple 2 doit être 64 et est : {}", ex_output2);
    println!("L'output 2 est : {} [Exécuté en : {:?}]", output2, rt2);  
}

fn part1(texte: &str) -> u64 {
    let mut score: u64 = 0;
    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let carac: Vec<char> = line.chars().collect();
        mat.push(carac);
    }
    
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 'O' {
                score += remonte(&mut mat, i, j);
            }
        }
    }

    return score
}

fn echange(mat: &mut Vec<Vec<char>>, n: usize, m: usize, i: usize, j : usize) -> () {
    let buffer = mat[n][m];
    mat[n][m] = mat[i][j];
    mat[i][j] = buffer;
}

fn remonte(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> u64 {
    if i == 0 || mat[i-1][j] == '#' || mat[i-1][j] == 'O'  {
        return (mat.len() - i) as u64;
    }
    else {
        echange(mat, i-1, j, i, j);
        return remonte(mat, i-1, j);
    }
}

fn part2(texte: &str) -> u64 {
    let mut score: u64 = 0;

    let nb_rep: usize = 1000000000;

    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let carac: Vec<char> = line.chars().collect();
        mat.push(carac);
    }

    let mut dict: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut k: usize = 0;

    while k < nb_rep {
        println!("{k}");
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 'O' {
                    spin_north(&mut mat, i, j);
                }
            }
        }
        for i in 0..mat[0].len() {
            for j in 0..mat.len() {
                if mat[j][i] == 'O' {
                    spin_west(&mut mat, j, i);
                }
            }
        }
        for i in (0..mat.len()).rev() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 'O' {
                    spin_south(&mut mat, i, j);
                }
            }
        }
        for i in (0..mat[0].len()).rev() {
            for j in 0..mat.len() {
                if mat[j][i] == 'O' {
                    spin_east(&mut mat, j, i);
                }
            }
        }
        let key: Vec<Vec<char>> = mat.clone();

        if dict.contains_key(&key) {
            let value = match dict.get(&key) {
                Some(x) => x,
                None => continue,
            };
            let diff = k - value;
            while k + diff < nb_rep {
                k += diff;
            }
        }
        else {
            dict.insert(key, k);
        }
        k += 1;
    }
    for j in 0..mat.len() {
        println!("{:?}", mat[j]);
    }
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 'O' {
                score += (mat.len() - i) as u64;
            }
        }
    }
    return score
}

fn spin_north(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> () {
    if i != 0 && mat[i-1][j] != '#' && mat[i-1][j] != 'O'  {
        echange(mat, i-1, j, i, j);
        spin_north(mat, i-1, j);
    }
}

fn spin_west(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> () {
    if j != 0 && mat[i][j-1] != '#' && mat[i][j-1] != 'O' {
        echange(mat, i, j-1, i, j);
        spin_west(mat, i, j-1);
    }
}

fn spin_south(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> () {
    if i != mat.len()-1 && mat[i+1][j] != '#' && mat[i+1][j] != 'O' {
        echange(mat, i+1, j, i, j);
        spin_south(mat, i+1, j);
    }
}

fn spin_east(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> () {
    if j != mat[0].len()-1 && mat[i][j+1] != '#' && mat[i][j+1] != 'O' {
        echange(mat, i, j+1, i, j);
        spin_east(mat, i, j+1);
    }
}
