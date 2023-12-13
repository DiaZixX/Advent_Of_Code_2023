use std::{collections::HashMap, time::Instant};

fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: u64 = part1(ex_input);

    let st1 = Instant::now();
    let output1: u64 = part1(input);
    let et1 = Instant::now();
    let rt1 = et1 - st1;
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output2: u64 = part2(ex_input);

    let st2 = Instant::now();
    let output2: u64 = part2(input);
    let et2 = Instant::now();
    let rt2 = et2 - st2;

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 21 et est : {}", ex_output1);
    println!("L'output 1 est : {} [Exécuté en : {:?}]", output1, rt1);  
    println!("L'output de l'exemple 2 doit être 525152 et est : {}", ex_output2);
    println!("L'output 2 est : {} [Exécuté en : {:?}]", output2, rt2);  
}

fn part1(texte: &str) -> u64 {
    let lines = texte.lines();
    let mut total = 0;
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let cfg = split[0];
        let mut nums : Vec<u32> = Vec::new();
        let numbers: Vec<&str> = split[1].split(',').collect();
        for i in 0..numbers.len() {
            match numbers[i].parse() {
                Ok(x) => nums.push(x),
                Err(_) => continue,
            }
        }

        // println!("{:?} {:?}", cfg, nums);
        total += count(cfg, &mut nums);
        // println!("{total}");
    }
    return total;
}

fn count(cfg: &str, nums: &mut Vec<u32>) -> u64 {
    if cfg.eq("") {
        if nums.is_empty() {
            return 1
        }
        else {
            return 0
        }
    }
    if nums.is_empty() {
        if cfg.contains("#") {
            return  0
        }
        else {
            return 1
        }
    }
    let mut result = 0;
    
    // println!("1ere {:?} {:?}", cfg, nums);
    if cfg[0..1].eq(".") || cfg[0..1].eq("?") {
        result += count(&cfg[1..], nums)
    }
    // println!("2eme {:?} {:?}", cfg, nums);
    if cfg[0..1].eq("#") || cfg[0..1].eq("?") {
        if nums[0] <= cfg.len() as u32 && !(cfg[..nums[0] as usize].contains(".")) && (nums[0] == cfg.len() as u32 || !(cfg[nums[0] as usize..nums[0] as usize+1].eq("#"))) {
            let temp = nums.remove(0);
            if temp + 1 >= cfg.len() as u32 {
                result += count("", nums);
            }
            else {
                result += count(&cfg[temp as usize +1..], nums);
            }
            nums.insert(0, temp);
        }
    }
    return result
}

fn part2(texte: &str) -> u64 {
    let lines = texte.lines();
    let mut total = 0;
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let cfg = split[0];
        let mut nums : Vec<u32> = Vec::new();
        let numbers: Vec<&str> = split[1].split(',').collect();
        for i in 0..numbers.len() {
            match numbers[i].parse() {
                Ok(x) => nums.push(x),
                Err(_) => continue,
            }
        }

        let mut cfg2: String = String::from(cfg);
        let mut nums2: Vec<u32> = nums.repeat(5);
        cfg2.push('?');
        cfg2.push_str(&cfg);
        cfg2.push('?');
        cfg2.push_str(&cfg);
        cfg2.push('?');
        cfg2.push_str(&cfg);
        cfg2.push('?');
        cfg2.push_str(&cfg);

        // println!("{:?} {:?}", cfg2, nums2);
        let mut dict : HashMap<(&str, Vec<u32>), u64> = HashMap::new();
        total += count2(&cfg2,&mut nums2, &mut dict);
        // println!("{total}");
    }
    return total;
}

fn count2<'a>(cfg: &'a str, nums: &mut Vec<u32>, dict : &mut HashMap<(&'a str, Vec<u32>), u64>) -> u64 {
    if cfg.eq("") {
        if nums.is_empty() {
            return 1
        }
        else {
            return 0
        }
    }
    if nums.is_empty() {
        if cfg.contains("#") {
            return  0
        }
        else {
            return 1
        }
    }

    let temp = nums.clone();

    let key = (cfg, temp);

    if dict.contains_key(&key) {
        match dict.get(&key) {
            Some(x) => return *x,
            None => (),
        }
    }

    let mut result = 0;
    
    // println!("1ere {:?} {:?}", cfg, nums);
    if cfg[0..1].eq(".") || cfg[0..1].eq("?") {
        result += count2(&cfg[1..], nums, dict)
    }
    // println!("2eme {:?} {:?}", cfg, nums);
    if cfg[0..1].eq("#") || cfg[0..1].eq("?") {
        if nums[0] <= cfg.len() as u32 && !(cfg[..nums[0] as usize].contains(".")) && (nums[0] == cfg.len() as u32 || !(cfg[nums[0] as usize..nums[0] as usize+1].eq("#"))) {
            let temp = nums.remove(0);
            if temp + 1 >= cfg.len() as u32 {
                result += count2("", nums, dict);
            }
            else {
                result += count2(&cfg[temp as usize +1..], nums, dict);
            }
            nums.insert(0, temp);
        }
    }
    dict.insert(key, result);
    return result
}
