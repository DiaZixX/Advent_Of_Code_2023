fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: u32 = part2(ex_input);
    let output2: u32 = part2(input);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 6440 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 5905 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let lines: Vec<&str> = texte.lines().collect();

    let nb_hands = lines.len();
    let mut hands: Vec<&str> = vec![""; nb_hands];
    let mut bids: Vec<u32> = vec![0; nb_hands];
    let mut type_hand: Vec<u32> = vec![0; nb_hands];


    //Remplissage des tableaux
    for i in 0..nb_hands {
        let temp: Vec<&str> = lines[i].split(' ').collect();
        hands[i] = temp[0];
        bids[i] = match temp[1].parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        type_hand[i] = type_of_hand(hands[i]);
    }

    //Tri insertion sur type_of_hand
    for i in 1..nb_hands {
        let en_cours = type_hand[i];
        let val_bid = bids[i];
        let val_hands = hands[i];

        if i < 4 {
            println!("Challenger {i} {:?} {:?} {:?}", val_hands, val_bid, en_cours);
            println!("Les mains : {:?}", hands);
        }

        let mut keep_j = 0;

        for j in (1..=i).rev() {
            if type_hand[j-1] > en_cours {
                type_hand[j] = type_hand[j-1];
                hands[j] = hands[j-1];
                bids[j] = bids[j-1];
            }
            else if type_hand[j-1] == en_cours {
                if high_card(val_hands, hands[j-1]) == 1 {
                    keep_j = j;
                    break;
                }
                else {
                    type_hand[j] = type_hand[j-1];
                    hands[j] = hands[j-1];
                    bids[j] = bids[j-1];
                }
            }
            else {
                keep_j = j;
                break;
            }
        }
        type_hand[keep_j] = en_cours;
        hands[keep_j] = val_hands;
        bids[keep_j] = val_bid;
    }

    println!("FINAL");
    println!("{:?} {:?} {:?}", hands, bids, type_hand);

    //Calcul du score
    let mut score: u32 = 0;

    for i in 0..nb_hands {
        score += ((i as u32)+1) * bids[i];
    }

    return score;
}

fn type_of_hand(hand: &str) -> u32 {
    //Calcule le type de main et renvoie son numero associé
    let mut chars: Vec<char> = hand.chars().collect();

    let mut i : usize = 0;

    loop {
        if i == 4 {
            return 0;
        }
        let c: char = chars[i];
        let nb_occur = chars.iter().filter(|&&x| x == c).count();
        if nb_occur == 5 {
            return 6;
        }
        else if nb_occur == 4 {
            return 5;
        }
        else if nb_occur == 3 {
            chars.retain(|&x| x != c);
            let nb_occur = chars.iter().filter(|&&x| x == chars[0]).count();
            if nb_occur == 2 {
                return 4;
            }
            else {
                return 3;
            }
        }
        else if nb_occur == 2 {
            chars.retain(|&x| x != c);
            let nb_occur_1 = chars.iter().filter(|&&x| x == chars[0]).count();
            let nb_occur_2 = chars.iter().filter(|&&x| x == chars[1]).count();
            if nb_occur_1 == 3 {
                return 4;
            }
            else if nb_occur_1 == 2 || nb_occur_2 == 2 {
                return 2;
            }
            else {
                return 1;
            }
        }
        i += 1;
    }
}

fn high_card(hand1: &str, hand2: &str) -> u32 {
    let cards1: Vec<char> = hand1.chars().collect();
    let cards2: Vec<char> = hand2.chars().collect();

    for i in 0..5 {
        let int_card1 = match cards1[i].to_digit(10) {
            Some(x) => x,
            None => {
                if cards1[i] == 'T' {
                    10
                }
                else if cards1[i] == 'J' {
                    11
                }
                else if cards1[i] == 'Q' {
                    12
                }
                else if cards1[i] == 'K' {
                    13
                }
                else if cards1[i] == 'A' {
                    14
                }
                else {
                    0
                }
            }
        };
        let int_card2 = match cards2[i].to_digit(10) {
            Some(x) => x,
            None => {
                if cards2[i] == 'T' {
                    10
                }
                else if cards2[i] == 'J' {
                    11
                }
                else if cards2[i] == 'Q' {
                    12
                }
                else if cards2[i] == 'K' {
                    13
                }
                else if cards2[i] == 'A' {
                    14
                }
                else {
                    0
                }
            }
        };

        if int_card1 > int_card2 {
            return 1;
        }
        else if int_card1 < int_card2 {
            return 2;
        }
        else {
            continue;
        }
    }
    return 0;
}









fn part2(texte: &str) -> u32 {
    let lines: Vec<&str> = texte.lines().collect();

    let nb_hands = lines.len();
    let mut hands: Vec<&str> = vec![""; nb_hands];
    let mut bids: Vec<u32> = vec![0; nb_hands];
    let mut type_hand: Vec<u32> = vec![0; nb_hands];


    //Remplissage des tableaux
    for i in 0..nb_hands {
        let temp: Vec<&str> = lines[i].split(' ').collect();
        hands[i] = temp[0];
        bids[i] = match temp[1].parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        type_hand[i] = type_of_hand2(hands[i]);
    }

    //Tri insertion sur type_of_hand
    for i in 1..nb_hands {
        let en_cours = type_hand[i];
        let val_bid = bids[i];
        let val_hands = hands[i];

        if i < 4 {
            println!("Challenger {i} {:?} {:?} {:?}", val_hands, val_bid, en_cours);
            println!("Les mains : {:?}", hands);
        }

        let mut keep_j = 0;

        for j in (1..=i).rev() {
            if type_hand[j-1] > en_cours {
                type_hand[j] = type_hand[j-1];
                hands[j] = hands[j-1];
                bids[j] = bids[j-1];
            }
            else if type_hand[j-1] == en_cours {
                if high_card2(val_hands, hands[j-1]) == 1 {
                    keep_j = j;
                    break;
                }
                else {
                    type_hand[j] = type_hand[j-1];
                    hands[j] = hands[j-1];
                    bids[j] = bids[j-1];
                }
            }
            else {
                keep_j = j;
                break;
            }
        }
        type_hand[keep_j] = en_cours;
        hands[keep_j] = val_hands;
        bids[keep_j] = val_bid;
    }

    println!("FINAL");
    println!("{:?} {:?} {:?}", hands, bids, type_hand);

    //Calcul du score
    let mut score: u32 = 0;

    for i in 0..nb_hands {
        score += ((i as u32)+1) * bids[i];
    }

    return score;
}

fn type_of_hand2(hand: &str) -> u32 {
    //Calcule le type de main et renvoie son numero associé
    let mut chars: Vec<char> = hand.chars().collect();

    let mut i : usize = 0;

    loop {
        if i == 4 {
            let nb_occur_j = chars.iter().filter(|&&x| x == 'J').count();
            if nb_occur_j == 5 || nb_occur_j == 4 {
                return 6;
            }
            else if nb_occur_j == 3 {
                return 5;
            }
            else if nb_occur_j == 2 {
                return 3;
            }
            else if nb_occur_j == 1 {
                return 1;
            }
            else {
                return 0;
            }
        }
        let c: char = chars[i];
        if c == 'J' {
            i += 1;
            continue;
        }
        let nb_occur = chars.iter().filter(|&&x| x == c).count();
        //Fait
        if nb_occur == 5 {
            return 6;
        }
        //Fait
        else if nb_occur == 4 {
            chars.retain(|&x| x != c);
            if chars[0] == 'J' {
                return 6;
            }
            else {
                return 5;
            }
        }
        //Fait 
        else if nb_occur == 3 {
            chars.retain(|&x| x != c);  //Rest 2 carte 
            let nb_occur_j = chars.iter().filter(|&&x| x == 'J').count();
            if nb_occur_j == 2 {
                return 6;
            }
            else if nb_occur_j == 1 {
                return 5;
            }
            else {
                let nb_occur = chars.iter().filter(|&&x| x == chars[0]).count();
                if nb_occur == 2 {
                    return 4;
                }
                else {
                    return 3;
                }
            }
        }
        // Fait 
        else if nb_occur == 2 {
            chars.retain(|&x| x != c);  //Reste 3 
            let nb_occur_j = chars.iter().filter(|&&x| x == 'J').count();
            if nb_occur_j == 3 {
                return 6;
            }
            else if nb_occur_j == 2 {
                return 5;
            }
            else if nb_occur_j == 1{
                chars.retain(|&x| x != 'J');  //Reste 2 
                let nb_occur_1 = chars.iter().filter(|&&x| x == chars[0]).count();
                if nb_occur_1 == 2 {
                    return 4
                }
                else {
                    return 3;
                }
            }
            else {
                let nb_occur_1 = chars.iter().filter(|&&x| x == chars[0]).count();
                let nb_occur_2 = chars.iter().filter(|&&x| x == chars[1]).count();
                if nb_occur_1 == 3 {
                    return 4;
                }
                else if nb_occur_1 == 2 || nb_occur_2 == 2 {
                    return 2;
                }
                else {
                    return 1;
                }
            }
        }
        i += 1;
    }
}

fn high_card2(hand1: &str, hand2: &str) -> u32 {
    let cards1: Vec<char> = hand1.chars().collect();
    let cards2: Vec<char> = hand2.chars().collect();

    for i in 0..5 {
        let int_card1 = match cards1[i].to_digit(10) {
            Some(x) => x,
            None => {
                if cards1[i] == 'T' {
                    10
                }
                else if cards1[i] == 'J' {
                    1
                }
                else if cards1[i] == 'Q' {
                    12
                }
                else if cards1[i] == 'K' {
                    13
                }
                else if cards1[i] == 'A' {
                    14
                }
                else {
                    0
                }
            }
        };
        let int_card2 = match cards2[i].to_digit(10) {
            Some(x) => x,
            None => {
                if cards2[i] == 'T' {
                    10
                }
                else if cards2[i] == 'J' {
                    1
                }
                else if cards2[i] == 'Q' {
                    12
                }
                else if cards2[i] == 'K' {
                    13
                }
                else if cards2[i] == 'A' {
                    14
                }
                else {
                    0
                }
            }
        };

        if int_card1 > int_card2 {
            return 1;
        }
        else if int_card1 < int_card2 {
            return 2;
        }
        else {
            continue;
        }
    }
    return 0;
}