fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    let ex_output2: u32 = part2(ex_input);
    let output2: u32 = part2(input);

    println!("L'output de l'exemple 1 doit être 13 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 30 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let mots = texte.lines();
    
    let mut score : u32 = 0;

    for mot in mots {
        let first_split: Vec<&str> = mot.split(':').collect();

        let mut winnings: Vec<u32> = Vec::new();
        let mut ours: Vec<u32> = Vec::new();

        let second_split: Vec<&str> = first_split[1].split('|').collect();

        let winning_numbers: Vec<&str> = second_split[0].split(' ').collect();
        let our_numbers: Vec<&str> = second_split[1].split(' ').collect();

        for i in winning_numbers {
            match i.parse() {
                Ok(num) => winnings.push(num),
                Err(_) => continue,
            };
        }
        for j in our_numbers {
            match j.parse() {
                Ok(num) => ours.push(num),
                Err(_) => continue,
            };
        }
            
        println!("{:?}", winnings);
        println!("{:?}", ours);

        let mut score_ligne : u32 = 0;
        
        while !ours.is_empty() {
            let temp = match ours.pop() {
                Some(x) => x,
                None => 10000,
            };
            if winnings.contains(&temp) {
                if score_ligne == 0 {
                    score_ligne += 1;
                }
                else {
                    score_ligne *= 2;
                }
            }
        }

        println!("Score de la carte : {}", score_ligne);

        score += score_ligne;

        println!("===== (CARD) =====");
    }

    return score
}

fn part2(texte: &str) -> u32 {
    let mots: std::str::Lines<'_> = texte.lines();
    
    let mots: Vec<&str>= mots.collect();
    let nb_cards = mots.len();

    let mots = mots.into_iter();

    let mut score : u32 = 0;
    let mut actual_card: u32 = 0;

    let mut cards = vec![1; nb_cards];

    for mot in mots {
        println!("===== (CARD {}) =====", actual_card+1);
        let first_split: Vec<&str> = mot.split(':').collect();

        let mut winnings: Vec<u32> = Vec::new();
        let mut ours: Vec<u32> = Vec::new();

        let second_split: Vec<&str> = first_split[1].split('|').collect();

        let winning_numbers: Vec<&str> = second_split[0].split(' ').collect();
        let our_numbers: Vec<&str> = second_split[1].split(' ').collect();

        for i in winning_numbers {
            match i.parse() {
                Ok(num) => winnings.push(num),
                Err(_) => continue,
            };
        }
        for j in our_numbers {
            match j.parse() {
                Ok(num) => ours.push(num),
                Err(_) => continue,
            };
        }
            
        println!("{:?}", winnings);
        println!("{:?}", ours);

        let mut score_ligne : u32 = 0;
        
        while !ours.is_empty() {
            let temp = match ours.pop() {
                Some(x) => x,
                None => 10000,
            };
            if winnings.contains(&temp) {
                score_ligne += 1;
            }
        }

        println!("Score de la carte : {}", score_ligne);

        for i in (actual_card as usize)+1..(actual_card as usize)+1+(score_ligne as usize) {
            cards[i] += cards[actual_card as usize] as i32;
        }

        println!("----");

        actual_card+=1;
    }

    println!("Cards : {:?}", cards);

    for i in 0..nb_cards {
        score += cards[i] as u32
    }

    return score
}