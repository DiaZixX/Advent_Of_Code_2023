use std::collections::HashMap;
use num::Integer;

fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input1: &str = include_str!("ex_input1.txt");
    let ex_input2: &str = include_str!("ex_input2.txt");
    let ex_input3: &str = include_str!("ex_input3.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: u32 = part1(ex_input1);
    let ex_output2: u32 = part1(ex_input2);
    let output1: u32 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    // let ex_output2: u32 = part2(ex_input);
    let ex_output3: u64 = part2(ex_input3);
    let output2: u64 = part2(input);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 2 et est : {}", ex_output1);
    println!("L'output de l'exemple 2 doit être 6 et est : {}", ex_output2);
    println!("L'output 1 est : {}", output1);
    println!("L'output de l'exemple 3 doit être 6 et est : {}", ex_output3);  
    // println!("L'output de l'exemple 2 doit être 5905 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let mut score: u32 = 0;

    let mut mon_dico: HashMap<&str,(&str,&str)> = HashMap::new();

    let morceaux : Vec<&str> = texte.lines().collect();
    //Remplissage du dico
    for i in 2..morceaux.len() {
        let direction : Vec<&str> = morceaux[i].split('=').collect();

        let cle = direction[0].trim();

        let elem = direction[1]; 
        let couple: (&str, &str) = (&elem[2..5], &elem[7..10]);

        mon_dico.insert(cle, couple);
    }

    let sequence: Vec<char> = morceaux[0].chars().collect();
    let mut sortie: bool = false;
    let mut state = "AAA";

    loop {
        if sortie {
            break;
        }
        else {
            
            for i in 0..sequence.len() {
                // println!("{i}");
                if state.eq("ZZZ") {
                    sortie = true;
                    break;
                }
                else {
                    // println!("{:?} {:?}", sequence[i], mon_dico.get(state));
                    if sequence[i] == 'L' {
                        state = match mon_dico.get(state) {
                            Some(x) => x.0,
                            None => continue,
                        };
                    }
                    else {
                        state = match mon_dico.get(state) {
                            Some(x) => x.1,
                            None => continue,
                        };
                    }
                    score += 1;
                }
            }
        }
    }

    return score
}

fn part2(texte: &str) -> u64 {
    let mut mon_dico: HashMap<&str,(&str,&str)> = HashMap::new();
    let mut starting_state: Vec<&str> = Vec::new();

    let morceaux : Vec<&str> = texte.lines().collect();
    //Remplissage du dico
    for i in 2..morceaux.len() {
        //Dico
        let direction : Vec<&str> = morceaux[i].split('=').collect();
        let cle = direction[0].trim();
        let elem = direction[1]; 
        let couple: (&str, &str) = (&elem[2..5], &elem[7..10]);
        mon_dico.insert(cle, couple);
        //Start state
        if cle[2..3].eq("A") {
            starting_state.push(cle);
        }

    }
    
    println!("Start state {:?}", starting_state);
    let nb_states = starting_state.len();
    let mut score_state: Vec<u32> = vec![0; nb_states];
    let sequence: Vec<char> = morceaux[0].chars().collect();

    //Parcours
    for k in 0..nb_states {
        let mut sortie: bool = false;
        let mut state = starting_state[k];
        loop {
            if sortie {
                break;
            }
            else {
                for i in 0..sequence.len() {
                    // println!("{i}");
                    if state[2..3].eq("Z") {
                        sortie = true;
                        break;
                    }
                    else {
                        // println!("{:?} {:?}", sequence[i], mon_dico.get(state));
                        if sequence[i] == 'L' {
                            state = match mon_dico.get(state) {
                                Some(x) => x.0,
                                None => continue,
                            };
                        }
                        else {
                            state = match mon_dico.get(state) {
                                Some(x) => x.1,
                                None => continue,
                            };
                        }
                        score_state[k] += 1;
                    }
                }
            }
        }
    }
    println!("Score state {:?}", score_state);

    let mut score: u64 = score_state[0] as u64;
    for i in 1..nb_states {
        score = pcm(score_state[i] as u64, score);
    }

    return score;
}

fn pgcd(a: u64, b: u64) -> u64 {
    a.gcd(&b)
}

fn pcm(a: u64, b: u64) -> u64 {
    a * b / pgcd(a, b)
}
