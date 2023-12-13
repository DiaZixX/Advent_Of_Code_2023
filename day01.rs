fn main() {
    let input: &str = include_str!("input1.txt");
    //println!("L'input est :  {}", input);
    let output1: u32 = part1(&input);
    println!("L'output 1 est : {}", output1); //La bonne réponse est 55621 fini a 06:45
    let output2: u32 = part2(&input);
    println!("L'output 2 est : {}", output2); //La bonne réponse est 53592 fini a 07:32
}

fn part1(input : &str) -> u32 {
    let bytes = &input.as_bytes();

    let mut cpt : usize = 0;
    let mut tot : u32 = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            tot = tot + find_value1(&input[cpt..i]);
            cpt = i+1;
        }
    }
    tot
}

fn part2(input : &str) -> u32 {
    let bytes = &input.as_bytes();

    let mut cpt : usize = 0;
    let mut tot : u32 = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            tot = tot + find_value2(&input[cpt..i]);
            cpt = i+1;
        }
    }
    tot
}

fn find_value1(texte : &str) -> u32{
    let mut nb_digit = 0;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    for letter in texte.chars() {
        let n = letter as u32 - 0x30;
        if n <= 9 {
            nb_digit += 1;
            if nb_digit == 1 {
                first_digit = n;
            }
            else {
                last_digit = n;
            }
        }
    }
    if nb_digit == 1 {
        return 11 * first_digit
    }
    else {
        return 10*first_digit + last_digit
    }
}

fn find_value2(texte : &str) -> u32 {
    let longueur: usize = str::len(texte);

    let mut nb_digit = 0;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    let mut idx: usize = 0;
    for letter in texte.chars() {
        let n = letter as u32 - 0x30;
        if n <= 9 {
            nb_digit += 1;
            if nb_digit == 1 {
                first_digit = n;
            }
            else {
                last_digit = n;
            }
        }
        else{
            let temp : u32 = written_digit(texte, idx, longueur);
            if temp != 0 {
                nb_digit += 1;
                if nb_digit == 1 {
                    first_digit = temp;
                }
                else {
                    last_digit = temp;
                }
            }
        }
        idx += 1;
    }
    if nb_digit == 1 {
        return 11 * first_digit
    }
    else {
        return 10*first_digit + last_digit
    }
}

fn written_digit(texte : &str, idx : usize, longueur : usize) -> u32 {
    if idx+2 <= longueur-1 && texte[idx..idx+3].eq("one") {
        return 1
    }
    else if idx+2 <= longueur-1 && texte[idx..idx+3].eq("two") {
        return 2
    }
    else if idx+4 <= longueur-1 && texte[idx..idx+5].eq("three") {
        return 3
    }
    else if idx+3 <= longueur-1 && texte[idx..idx+4].eq("four") {
        return 4
    }
    else if idx+3 <= longueur-1 && texte[idx..idx+4].eq("five") {
        return 5
    }
    else if idx+2 <= longueur-1 && texte[idx..idx+3].eq("six") {
        return 6
    }
    else if idx+4 <= longueur-1 && texte[idx..idx+5].eq("seven") {
        return 7
    }
    else if idx+4 <= longueur-1 && texte[idx..idx+5].eq("eight") {
        return 8
    }
    else if idx+3 <= longueur-1 && texte[idx..idx+4].eq("nine") {
        return 9
    }
    else {
        return 0
    }
}
