use std::time::Instant;

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

    println!("L'output de l'exemple 1 doit être 46 et est : {}", ex_output1);
    println!("L'output 1 est : {} [Exécuté en : {:?}]", output1, rt1);  
    println!("L'output de l'exemple 2 doit être 51 et est : {}", ex_output2);
    println!("L'output 2 est : {} [Exécuté en : {:?}]", output2, rt2);  
}

fn part1(texte: &str) -> u64 {
    let lines = texte.lines();

    let mut mat : Vec<Vec<char>> = Vec::new();
    let mut filling : Vec<Vec<char>> = Vec::new();

    for line in lines {
        let carac_line : Vec<char> = line.chars().collect();
        let longueur = carac_line.len();
        let points: Vec<char> = vec!['.'; longueur];
        mat.push(carac_line);
        filling.push(points);
    }

    propage(&mat, &mut filling, 0, 0, 'L', max(mat.len(), mat[0].len()) as u32);

    let score: u64 = count_filling(&filling);

    return score
}

fn part2(texte: &str) -> u64 {
    let mut score : u64 = 0;

    let lines = texte.lines();

    let mut mat : Vec<Vec<char>> = Vec::new();
    let mut filling : Vec<Vec<char>> = Vec::new();

    for line in lines {
        let carac_line : Vec<char> = line.chars().collect();
        let longueur = carac_line.len();
        let points: Vec<char> = vec!['.'; longueur];
        mat.push(carac_line);
        filling.push(points);
    }

    for i in 0..filling.len() {
        propage(&mat, &mut filling, i as isize, 0, 'L', max(mat.len(), mat[0].len()) as u32);
        let temp_score = count_filling(&filling);
        if temp_score > score {
            score = temp_score;
        }
        reset_filling(&mut filling);
    }
    for i in 0..filling.len() {
        propage(&mat, &mut filling, i as isize, (mat[0].len() - 1) as isize, 'R', max(mat.len(), mat[0].len()) as u32);
        let temp_score = count_filling(&filling);
        if temp_score > score {
            score = temp_score;
        }
        reset_filling(&mut filling);
    }
    for j in 0..filling[0].len() {
        propage(&mat, &mut filling, 0, j as isize, 'U', max(mat.len(), mat[0].len()) as u32);
        let temp_score = count_filling(&filling);
        if temp_score > score {
            score = temp_score;
        }
        reset_filling(&mut filling);

    }
    for j in 0..filling[0].len() {
        propage(&mat, &mut filling, (mat.len()-1) as isize, j as isize, 'U', max(mat.len(), mat[0].len()) as u32);
        let temp_score = count_filling(&filling);
        if temp_score > score {
            score = temp_score;
        }
        reset_filling(&mut filling);  
    }

    return score
}

fn propage(mat : &Vec<Vec<char>>, fill : &mut Vec<Vec<char>>, i : isize, j : isize, from : char, ttl : u32) -> () {
    if i < 0 || j < 0 || i >= mat.len() as isize || j >= mat[0].len() as isize || ttl == 0 {
        return
    }
    else {
        let replaced = have_to_replace(fill, i as usize, j as usize);
        let mut new_ttl = ttl - 1;
        if replaced {
            new_ttl = max(mat.len(), mat[0].len()) as u32;
        }
        if from == 'L' {
            if mat[i as usize][j as usize] == '\\' {
                propage(mat, fill, i+1, j, 'U', new_ttl)
            }
            else if mat[i as usize][j as usize] == '/' {
                propage(mat, fill, i-1, j, 'D', new_ttl)
            }
            else if mat[i as usize][j as usize] == '-' {
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
            else if mat[i as usize][j as usize] == '|' {
                propage(mat, fill, i+1, j, 'U', new_ttl);
                propage(mat, fill, i-1, j, 'D', new_ttl)
            }
            else {
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
        }
        else if from == 'R' {
            if mat[i as usize][j as usize] == '\\' {
                propage(mat, fill, i-1, j, 'D', new_ttl)
            }
            else if mat[i as usize][j as usize] == '/' {
                propage(mat, fill, i+1, j, 'U', new_ttl)
            }
            else if mat[i as usize][j as usize] == '-' {
                propage(mat, fill, i, j-1, 'R', new_ttl)
            }
            else if mat[i as usize][j as usize] == '|' {
                propage(mat, fill, i-1, j, 'D', new_ttl);
                propage(mat, fill, i+1, j, 'U', new_ttl)
            }
            else {
                propage(mat, fill, i, j-1, 'R', new_ttl)
            }
        }
        else if from == 'U' {
            if mat[i as usize][j as usize] == '\\' {
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
            else if mat[i as usize][j as usize] == '/' {
                propage(mat, fill, i, j-1, 'R', new_ttl)
            }
            else if mat[i as usize][j as usize] == '-' {
                propage(mat, fill, i, j-1, 'R', new_ttl);
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
            else if mat[i as usize][j as usize] == '|' {
                propage(mat, fill, i+1, j, 'U', new_ttl)
            }
            else {
                propage(mat, fill, i+1, j, 'U', new_ttl)
            }
        }
        else {
            if mat[i as usize][j as usize] == '\\' {
                propage(mat, fill, i, j-1, 'R', new_ttl)
            }
            else if mat[i as usize][j as usize] == '/' {
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
            else if mat[i as usize][j as usize] == '-' {
                propage(mat, fill, i, j-1, 'R', new_ttl);
                propage(mat, fill, i, j+1, 'L', new_ttl)
            }
            else if mat[i as usize][j as usize] == '|' {
                propage(mat, fill, i-1, j, 'D', new_ttl);
            }
            else {
                propage(mat, fill, i-1, j, 'D', new_ttl);
            }
        }
    }
}

fn have_to_replace(fill : &mut Vec<Vec<char>>, i : usize, j : usize) -> bool {
    if fill[i][j] == '.' {
        fill[i][j] = '#';
        return true
    }
    return false 
}

fn max(a: usize, b: usize) -> usize {
    if a >= b {
        return a
    }
    return b
}

fn reset_filling(fill : &mut Vec<Vec<char>>) -> () {
    for i in 0..fill.len() {
        for j in 0..fill[0].len() {
            fill[i][j] = '.';
        }
    }
}

fn count_filling(fill : &Vec<Vec<char>>) -> u64 {
    let mut score : u64 = 0;
    for i in 0..fill.len() {
        for j in 0..fill[0].len() {
            if fill[i][j] == '#' {
                score += 1;
            }
        }
    }
    return score
}
