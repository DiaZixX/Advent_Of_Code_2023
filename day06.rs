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

    println!("L'output de l'exemple 1 doit être 288 et est : {}", ex_output1);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 2 doit être 71503 et est : {}", ex_output2);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> u32 {
    let mut score: u32 = 0;

    let lines: Vec<&str> = texte.lines().collect();

    let morceau_time: Vec<&str> = lines[0].split(':').collect();
    let morceau_dist: Vec<&str> = lines[1].split(':').collect();

    let mut digits_times: Vec<&str> = morceau_time[1].split(' ').collect();
    let mut digits_distance: Vec<&str> = morceau_dist[1].split(' ').collect();

    digits_times.retain(|&x| x != "");
    digits_distance.retain(|&x| x != "");

    let nb_races = digits_times.len();

    let mut times = vec![0;nb_races];
    let mut distances = vec![0;nb_races];

    for i in 0..nb_races {
        times[i] = match digits_times[i].trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        distances[i] = match digits_distance[i].trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
    }

    println!("Times : {:?}", times);
    println!("Distances : {:?}", distances);

    for i in 0..nb_races {
        let mut score_race = 0;
        for sec in 1..times[i] as usize+1 {
            if (times[i] - sec) * sec > distances[i]{
                score_race += 1;
            }
        }
        println!("Race {}, score_race : {}", i+1, score_race);
        if score == 0 {
            score = score_race;
        }
        else {
            score *= score_race;
        }   
    }
    return score
}

fn part2(texte: &str) -> u32 {
    let mut score: u32 = 0;

    let lines: Vec<&str> = texte.lines().collect();

    let morceau_time: Vec<&str> = lines[0].split(':').collect();
    let morceau_dist: Vec<&str> = lines[1].split(':').collect();

    let mut digits_times: Vec<&str> = morceau_time[1].split(' ').collect();
    let mut digits_distance: Vec<&str> = morceau_dist[1].split(' ').collect();

    digits_times.retain(|&x| x != "");
    digits_distance.retain(|&x| x != "");

    let time: String = digits_times.iter().cloned().collect();
    let distance: String = digits_distance.iter().cloned().collect();

    let time: u64 = match time.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    let distance: u64 = match distance.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    println!("Time : {:?}", time);
    println!("Distance : {:?}", distance);

    for sec in 1..time+1 {
        if (time - sec) * sec > distance{
            score += 1;
        }
    }
    return score
}
