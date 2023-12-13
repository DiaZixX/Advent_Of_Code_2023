fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    //println!("L'input exemple est : {}", ex_input);
    //println!("L'input est :  {}", input);
    
    println!("<========== (PART 2) ==========>");

    let ex_output1: u32 = part1(ex_input);
    let output1: u32 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: u32 = part2(ex_input);
    let output2: u32 = part2(input);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 35 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 46 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let lines: Vec<&str> = texte.lines().collect();

    let graines: Vec<&str> = lines[0].split(':').collect();
    let graines: Vec<&str> = graines[1].split(' ').collect();

    let mut tab_graines: Vec<i64> = vec![0; graines.len()-1];

    for i in 1..graines.len() {
        tab_graines[i-1] = match graines[i].trim().parse(){
            Ok(x) => x,
            Err(_) => continue,
        };
    }

    println!("Tab_graines {:?}", tab_graines);
    let nb_graines = tab_graines.len();
    let mut etape = 3;
    let mut need_number = vec![true; nb_graines];

    while etape < lines.len() {
        //println!("Etape : {:?}, Line: {:?}, Tab : {:?}", etape, lines[etape], tab_graines);

        if lines[etape].eq("") {
            etape += 2;

            println!("{:?}", tab_graines);

            for i in 0..nb_graines {
                need_number[i] = true
            }
        }
        else {
            let range: Vec<&str> = lines[etape].split(' ').collect();

            let mut number = [0; 3];

            for i in 0..3 {
                number[i] = match range[i].trim().parse(){
                    Ok(x) => x,
                    Err(_) => continue,
                };
            }

            for i in 0..nb_graines {
                if need_number[i] && tab_graines[i] >= number[1] && tab_graines[i] < number[1] + number[2] {
                    tab_graines[i] += number[0] - number[1];
                    need_number[i] = false;
                }
            }

            etape += 1;
        }
    }

    let mut score = tab_graines[0];
    for i in 1..nb_graines {
        if tab_graines[i] < score {
            score = tab_graines[i];
        }
    }

    println!("{:?}", tab_graines);

    return score as u32;
}

fn part2(texte: &str) -> u32 {
    let lines: Vec<&str> = texte.lines().collect();

    let graines: Vec<&str> = lines[0].split(':').collect();
    let graines: Vec<&str> = graines[1].split(' ').collect();

    let mut tab_graines: Vec<i64> = Vec::new();
    let mut range: Vec<i64> = Vec::new();
    let mut need_number: Vec<bool> = Vec::new();

    for i in 1..graines.len() {
        let temp: i64 = match graines[i].trim().parse(){
            Ok(x) => x,
            Err(_) => continue,
        };
        if i % 2 == 1 {
            tab_graines.push(temp);
            need_number.push(true);
        }
        else {
            range.push(temp);
        }
    }
    
    println!("Nb_graines : {:?}, Tab_graines : {:?}, Range : {:?}",  tab_graines.len(), tab_graines, range);
    let mut etape = 3;

    while etape < lines.len() {
        //println!("Etape : {:?}, Line: {:?}, Tab : {:?}", etape, lines[etape], tab_graines);

        if lines[etape].eq("") {
            etape += 2;

            println!("Tab : {:?}, Range : {:?}", tab_graines, range);
            println!("===========Step============");

            for i in 0..tab_graines.len() {
                need_number[i] = true
            }
        }
        else {
            let range_line: Vec<&str> = lines[etape].split(' ').collect();

            let mut number = [0; 3];

            for i in 0..3 {
                number[i] = match range_line[i].trim().parse(){
                    Ok(x) => x,
                    Err(_) => continue,
                };
            }

            //Portion a modif

            let temp = tab_graines.len();

            for i in 0..temp {
                if need_number[i] {
                    //println!("Tab : {:?}, Range : {:?}, Need : {:?}, Line : {:?}", tab_graines, range, need_number, lines[etape]);
                    if (tab_graines[i] + range[i] - 1) < number[1] {
                        continue;
                    }
                    else if tab_graines[i] > (number[1] + number[2] - 1){
                        continue;
                    }
                    else if tab_graines[i] >= number[1] && (tab_graines[i] + range[i] - 1) <= (number[1] + number[2] - 1){
                        tab_graines[i] += number[0] - number[1];
                        need_number[i] = false;
                    }
                    else if tab_graines[i] < number[1] && (tab_graines[i] + range[i] - 1) >= number[1] && (tab_graines[i] + range[i] - 1) <= (number[1] + number[2] - 1){
                        //println!("Passage {} -> Tab : {:?}, Range : {:?}" , i, tab_graines, range);
                        tab_graines.push(tab_graines[i]);
                        range.push(number[1] - tab_graines[i]);
                        need_number.push(true);
                        range[i] -= number[1] - tab_graines[i];
                        tab_graines[i] = number[0];
                        need_number[i] = false;
                    }
                    else if tab_graines[i] >= number[1] && (tab_graines[i] + range[i] - 1) > (number[1] + number[2] - 1) && tab_graines[i] <= (number[1] + number[2] - 1) {
                        //println!("Passage {} -> Tab : {:?}, Range : {:?}" , i, tab_graines, range);
                        tab_graines.push(number[1] + number[2]);
                        range.push(tab_graines[i] + range[i] - number[1] - number[2]);
                        need_number.push(true);
                        range[i] = range[i] - (tab_graines[i] + range[i] - number[1] - number[2]);
                        tab_graines[i] += number[0] - number[1];
                        need_number[i] = false;
                    }
                    else {
                        println!("Passage");
                        tab_graines.push(tab_graines[i]);
                        range.push(number[1] - tab_graines[i]);
                        need_number.push(true);

                        tab_graines.push(number[1] + number[2]);
                        range.push(tab_graines[i] + range[i] - number[1] - number[2]);
                        need_number.push(true);

                        tab_graines[i] = number[0];
                        range[i] = number[2];
                        need_number[i] = false;
                    }
                }
            }

            etape += 1;
        }
    }
    println!("Tab : {:?}, Range : {:?}", tab_graines, range);

    let mut score = tab_graines[0];
    for i in 1..tab_graines.len() {
        if tab_graines[i] < score {
            score = tab_graines[i];
        }
    }

    return score as u32;
}
