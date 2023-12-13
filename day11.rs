use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: u64 = part2(ex_input, 10);
    let ex_output2_1: u64 = part2(ex_input, 100);
    let output2: u64 = part2(input, 1_000_000);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 374 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 1030 pour 10 et 8410 pour 100 est : {} pour 10 et {} pour 100", ex_output2, ex_output2_1);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let mut score: u32 = 0;

    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut only_point: bool = true;
        let mut temp: Vec<char> = Vec::new();
        for carac in line.chars() {
            if carac != '.' && only_point {
                only_point = false;
            }
            temp.push(carac);
        }
        mat.push(temp);
        if only_point {
            mat.push(vec!['.'; mat[0].len()]);
        }
    }

    let mut j: usize = 0;
    let mut nb_galaxie = 0;

    let mut dico: HashMap<u32, (usize, usize)> = HashMap::new();

    while j < mat[0].len() {
        let mut only_point = true;
        for i in 0..mat.len() {
            if mat[i][j] != '.' {
                if only_point {
                    only_point = false;
                }
                dico.insert(nb_galaxie,(i,j));
                nb_galaxie+=1;
            }
        }
        if only_point {
            for i in 0..mat.len() {
                mat[i].insert(j, '.');
            }
            j+=1;
        }
        j+=1;
    }

    for i in 0..nb_galaxie as usize {
        for j in i+1..nb_galaxie as usize {
            let coord1 = match dico.get(&(i as u32)) {
                Some(x) => x,
                None => continue,
            };
            let coord2 = match dico.get(&(j as u32)) {
                Some(x) => x,
                None => continue,
            };
            score += (coord1.0 as u32).abs_diff(coord2.0 as u32) + (coord1.1 as u32).abs_diff(coord2.1 as u32);
        }
    }

    return score;
}

fn part2(texte: &str, value_pas: u64) -> u64 {
    let mut score: u64 = 0;

    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut temp: Vec<char> = Vec::new();
        for carac in line.chars() {
            temp.push(carac);
        }
        mat.push(temp);
    }

    let mut nb_galaxie = 0;

    let mut dico: HashMap<u32, (usize, usize)> = HashMap::new();

    let mut ligne_ajout: Vec<usize> = vec![0;mat.len()];
    let mut colonne_ajout: Vec<usize> = vec![0;mat[0].len()];

    for i in 1..mat.len() {
        ligne_ajout[i] = i;
    }
    for j in 1..mat[0].len(){
        colonne_ajout[j] = j;
    }

    for j in 0..mat[0].len() {
        for i in 0..mat.len() {
            if mat[i][j] != '.' {
                dico.insert(nb_galaxie,(i,j));
                nb_galaxie+=1;
                ligne_ajout.retain(|&x| x != i);
                colonne_ajout.retain(|&x| x != j);
            }
        }
    }

    for i in 0..nb_galaxie as usize {
        for j in i+1..nb_galaxie as usize {
            let coord1 = match dico.get(&(i as u32)) {
                Some(x) => x,
                None => continue,
            };
            let coord2 = match dico.get(&(j as u32)) {
                Some(x) => x,
                None => continue,
            };
            let mut pas: u64 = 0;
            let ligne_crossed = is_crossed(coord1.0, coord2.0);
            let colonne_crossed = is_crossed(coord1.1, coord2.1);
            for i in &ligne_crossed{
                if ligne_ajout.contains(&i){
                    pas+=1;
                }
            }
            for j in &colonne_crossed {
                if colonne_ajout.contains(&j){
                    pas+=1;
                }
            }
            score += (coord1.0 as u64).abs_diff(coord2.0 as u64) + (coord1.1 as u64).abs_diff(coord2.1 as u64) + pas * value_pas - pas;
        }
    }
    return score;
}

fn is_crossed(i: usize, j: usize) -> Vec<usize> {
    let mut crossed: Vec<usize> = Vec::new();
    if i <= j {
        for k in i..j+1 {
            crossed.push(k)
        }
    }
    else {
        for k in j..i+1 {
            crossed.push(k)
        }
    }
    return crossed
}
