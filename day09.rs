fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: i64 = part1(ex_input);
    let output1: i64 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: i64 = part2(ex_input);
    let output2: i64 = part2(input);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 114 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 2 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> i64 {
    let mut score: i64 = 0;
    let lines: Vec<&str> = texte.lines().collect();

    for line in lines {
        let str_digits: Vec<&str> = line.split(' ').collect();

        let mut digits: Vec<i64> = Vec::new();

        for i in 0..str_digits.len() {
            match str_digits[i].parse() {
                Ok(x) => digits.push(x),
                Err(_) => continue,
            }
        }

        let mut derivations: Vec<Vec<i64>> = Vec::new();
        derivations.push(digits);
        let mut i = 0;

        //calcul des dérivations
        while !(full_zero(&derivations[i])) {
            let temp = derivate(&derivations[i]);
            derivations.push(temp);
            i+=1;
        }
        for j in (0..i+1).rev() {
            if j == i {
                derivations[j].push(0);
            }
            else {
                let value = derivations[j][derivations[j].len() - 1] + derivations[j+1][derivations[j+1].len() - 1]; 
                derivations[j].push(value);
                if j == 0 {
                    score += value;
                }
            }
        }
        println!("{:?} {i}", derivations);

    }
    return score;
}

fn part2(texte: &str) -> i64 {
    let mut score: i64 = 0;
    let lines: Vec<&str> = texte.lines().collect();

    for line in lines {
        let str_digits: Vec<&str> = line.split(' ').collect();

        let mut digits: Vec<i64> = Vec::new();

        for i in 0..str_digits.len() {
            match str_digits[i].parse() {
                Ok(x) => digits.push(x),
                Err(_) => continue,
            }
        }

        let mut derivations: Vec<Vec<i64>> = Vec::new();
        digits.reverse();
        derivations.push(digits);
        let mut i = 0;

        //calcul des dérivations
        while !(full_zero(&derivations[i])) {
            let temp = derivate2(&derivations[i]);
            derivations.push(temp);
            i+=1;
        }
        for j in (0..i+1).rev() {
            if j == i {
                derivations[j].push(0);
            }
            else {
                let value = derivations[j][derivations[j].len() - 1] - derivations[j+1][derivations[j+1].len() - 1]; 
                derivations[j].push(value);
                if j == 0 {
                    score += value;
                }
            }
        }
        println!("{:?} {i}", derivations);
    }
    return score;
}

fn derivate(tab: &Vec<i64>) -> Vec<i64> {
    let taille = tab.len();
    let mut deriv: Vec<i64> = vec![0; taille-1];
    for i in 0..taille-1  {
        deriv[i] = tab[i+1] - tab[i];
    }
    return deriv
}

fn derivate2(tab: &Vec<i64>) -> Vec<i64> {
    let taille = tab.len();
    let mut deriv: Vec<i64> = vec![0; taille-1];
    for i in 0..taille-1  {
        deriv[i] = tab[i] - tab[i+1];
    }
    return deriv
}

fn full_zero(tab: &Vec<i64>) -> bool {
    for i in tab {
        if *i != 0 {
            return false
        }
    }
    return true
}
