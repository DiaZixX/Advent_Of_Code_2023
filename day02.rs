fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    let ex_output2: u32 = part2(ex_input);
    let output2: u32 = part2(input);

    println!("L'output de l'exemple 1 est : {}", ex_output1);
    println!("L'output de l'exemple 2 est : {}", ex_output2);
    println!("L'output 1 est : {}", output1);  
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte : &str) -> u32{
    let mots = texte.lines();
    
    let mut ligne: u32 = 1;
    let mut score: u32 = 0;

    for mot in mots {

        let mut ligne_valide: bool = true;

        let first_split: Vec<&str> = mot.split(':').collect();

        let second_split: Vec<&str> = first_split[1].split(';').collect();
        for morceaux in second_split {

            let mut nb_red: u32 = 0;
            let mut nb_green: u32 = 0;
            let mut nb_blue: u32 = 0;  

            let third_split: Vec<&str> = morceaux.split(',').collect();
            for bloc in third_split {
                let fourth_split: Vec<&str> = bloc.split(' ').collect();
                let number: u32 = match fourth_split[1].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if fourth_split[2].eq("red") {
                    nb_red = number;
                }
                else if fourth_split[2].eq("green"){
                    nb_green = number;
                }
                else {
                    nb_blue = number;
                }
            }

            if !(nb_red <= 12 && nb_green <= 13 && nb_blue <= 14){
                ligne_valide = false;
            }
        }
        if ligne_valide {
            score += ligne;
            println!("Ligne {} valide", ligne);
        }
        println!("===================");
        ligne += 1;
    }

    return score;
}

fn part2(texte : &str) -> u32 {
    let mots = texte.lines();
    
    let mut score: u32 = 0;

    for mot in mots {

        let first_split: Vec<&str> = mot.split(':').collect();

        let mut min_red :u32 = 0;
        let mut min_green :u32 = 0;
        let mut min_blue :u32 = 0;

        let second_split: Vec<&str> = first_split[1].split(';').collect();
        for morceaux in second_split {

            let mut nb_red: u32 = 0;
            let mut nb_green: u32 = 0;
            let mut nb_blue: u32 = 0;  

            let third_split: Vec<&str> = morceaux.split(',').collect();
            for bloc in third_split {
                let fourth_split: Vec<&str> = bloc.split(' ').collect();
                let number: u32 = match fourth_split[1].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if fourth_split[2].eq("red") {
                    nb_red = number;
                }
                else if fourth_split[2].eq("green"){
                    nb_green = number;
                }
                else {
                    nb_blue = number;
                }
            }
            if nb_red > min_red {
                min_red = nb_red;
            }
            if nb_green > min_green {
                min_green = nb_green;
            }
            if nb_blue > min_blue {
                min_blue = nb_blue;
            }
        }
        score += min_blue * min_green * min_red;
    }
    return score;
}

