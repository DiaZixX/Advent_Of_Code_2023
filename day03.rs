fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    let ex_output2: u32 = part2(ex_input);
    let output2: u32 = part2(input);

    println!("L'output de l'exemple 1 doit être 4361 et est : {}", ex_output1);
    println!("L'output de l'exemple 2 doit être 467835 et est : {}", ex_output2);
    println!("L'output 1 est : {}", output1);  
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {

    let mut mat: Vec<Vec<char>> = Vec::new();

    let mut n: usize = 0;
    let mut m: usize = 0;

    let lignes: Vec<&str> = texte.lines().collect();

    for ligne in lignes {
        
        m = 0;
        let mut tab: Vec<char> = Vec::new();

        let caracteres = ligne.chars();
        for caractere in caracteres {
            tab.push(caractere);
            m += 1;
        }

        mat.push(tab);
        n += 1;
    }

    println!("Valeur de n et m : {} {}", n, m);

    let mut score: u32 = 0;
    let mut saut_1: bool = false;
    let mut saut_2: bool = false;

    for i in 0..n {
        for j in 0..m {
            if saut_1{
                saut_1 = false;
                continue;
            }
            if saut_2{
                saut_2 = false;
                continue;
            }
            if is_number(&mat, i, j) {
                let taille = check_right(&mat, i, j, m);
                let mut number: u32 = 0;
                
                if taille == 2 {
                    number += 100 * (mat[i][j] as u32 - 0x30) + 10 * (mat[i][j+1] as u32 - 0x30) + (mat[i][j+2] as u32 - 0x30);
                    saut_1 = true;
                    saut_2 = true;
                }
                else if taille == 1 {
                    number += 10 * (mat[i][j] as u32 - 0x30) + (mat[i][j+1] as u32 - 0x30);
                    saut_1 = true;
                }
                else {
                    number += mat[i][j] as u32 - 0x30;
                }

                println!("Valeur du nombre testé : {}", number);

                let mut special : bool = false;

                for a in ((i as isize)-1)..((i as isize)+2) {
                    for b in ((j as isize)-1)..((j as isize)+2+(taille as isize)){
                        if a >= 0 && a < n as isize && b >= 0 && b < m as isize {
                            if mat[a as usize][b as usize] != '.' && !is_number(&mat, a as usize, b as usize){
                                special = true;
                            }
                        }
                    }
                }

                if special{
                    println!("{} est spécial", number);
                    score += number;
                }

            }
                            
                        
        }
    }
        
    println!("==========");
    
    return score
}

fn part2(texte: &str) -> u32 {

    let mut mat: Vec<Vec<char>> = Vec::new();

    let mut n: usize = 0;
    let mut m: usize = 0;

    let lignes: Vec<&str> = texte.lines().collect();

    for ligne in lignes {
        
        m = 0;
        let mut tab: Vec<char> = Vec::new();

        let caracteres = ligne.chars();
        for caractere in caracteres {
            tab.push(caractere);
            m += 1;
        }

        mat.push(tab);
        n += 1;
    }

    println!("Valeur de n et m : {} {}", n, m);


    let mut score: u32 = 0;
    let mut saut_1: bool = false;
    let mut saut_2: bool = false;

    let mut geared: Vec<((usize,usize), u32)> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if saut_1{
                saut_1 = false;
                continue;
            }
            if saut_2{
                saut_2 = false;
                continue;
            }
            if is_number(&mat, i, j) {
                let taille = check_right(&mat, i, j, m);
                let mut number: u32 = 0;
                
                if taille == 2 {
                    number += 100 * (mat[i][j] as u32 - 0x30) + 10 * (mat[i][j+1] as u32 - 0x30) + (mat[i][j+2] as u32 - 0x30);
                    saut_1 = true;
                    saut_2 = true;
                }
                else if taille == 1 {
                    number += 10 * (mat[i][j] as u32 - 0x30) + (mat[i][j+1] as u32 - 0x30);
                    saut_1 = true;
                }
                else {
                    number += mat[i][j] as u32 - 0x30;
                }

                println!("Valeur du nombre testé : {}", number);


                for a in ((i as isize)-1)..((i as isize)+2) {
                    for b in ((j as isize)-1)..((j as isize)+2+(taille as isize)){
                        if a >= 0 && a < n as isize && b >= 0 && b < m as isize {
                            if mat[a as usize][b as usize] == '*' {
                                geared.push(((a as usize, b as usize), number))
                            }
                        }
                    }
                }

            }

        }
                            
                        
    }

    while !geared.is_empty() {
        let premier = geared.pop();

        let mut temp_score : u32 = 0;
        let mut temp_coord: (usize, usize) = (0,0);
        let mut binome: bool = false;

        match premier{
            Some((x,y)) => {
                temp_score = y;
                temp_coord = x
            },
            _ => println!("BIG PROBLEM"),
        }

        for i in 0..geared.len() {
            let temp = geared[i];
            match temp {
                (x, val) => { 
                    if x == temp_coord {
                        temp_score *= val;
                        binome = true;
                        geared.retain(|&x| x != temp);
                        break;
                    }
                    else {
                        continue;
                    }
                }
            }
        }
        if binome{
            score += temp_score;
        }
    }
    return score
}

fn check_right(mat: &Vec<Vec<char>>, i: usize, j: usize, m: usize) -> usize{
    if j+1 < m && is_number(mat, i, &j+1) {
        if j+2 < m && is_number(mat, i, &j+2) {
            return 2;
        }
        return 1;
    }
    return 0;
}

fn is_number(mat: &Vec<Vec<char>>, i: usize, j: usize) -> bool{
    let n: i32 = mat[i][j] as i32 - 0x30;
    n >= 0 && n <= 9 
}
