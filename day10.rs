fn main() {
    let input: &str = include_str!("input.txt");
    let ex_input: &str = include_str!("ex_input.txt");
    let ex_input2: &str = include_str!("ex_input2.txt");
    let ex_input3: &str = include_str!("ex_input3.txt");
    let ex_input4: &str = include_str!("ex_input4.txt");
    let ex_input5: &str = include_str!("ex_input5.txt");
    
    println!("<========== (PART 1) ==========>");

    let ex_output1: i64 = part1(ex_input);
    let ex_output2: i64 = part1(ex_input2);
    let output1: i64 = part1(input);
    
    println!("<========== (PART 2) ==========>");
    
    let ex_output3: i64 = part2(ex_input3);
    let ex_output4: i64 = part2(ex_input4);
    let ex_output5: i64 = part2(ex_input5);
    let output2: i64 = part2(input);

    println!("<========== (OUTPUT) ==========>");

    println!("L'output de l'exemple 1 doit être 4 et est : {}", ex_output1);
    println!("L'output de l'exemple 2 doit être 8 et est : {}", ex_output2);
    println!("L'output 1 est : {}", output1);  
    println!("L'output de l'exemple 3 doit être 4 et est : {}", ex_output3);
    println!("L'output de l'exemple 4 doit être 8 et est : {}", ex_output4);
    println!("L'output de l'exemple 5 doit être 10 et est : {}", ex_output5);
    println!("L'output 2 est : {}", output2);  
}

fn part1(texte: &str) -> i64 {
    let mut score: i64 = 0;

    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let morceaux = line.chars();
        let mut tab : Vec<char> = Vec::new();

        for carac in morceaux {
            tab.push(carac);
        }
        mat.push(tab);

    }

    //Find the S 
    let hauteur = mat.len();
    let largeur = mat[0].len();

    let mut current_point = find_s(&mat, hauteur, largeur);
    let pos_s = current_point;
    let next = look_around(&mat, hauteur, largeur, pos_s.0 as isize, pos_s.1 as isize);
    
    if next == 'H' {
        current_point = (current_point.0 - 1, current_point.1)
    }
    else if next == 'D' {
        current_point = (current_point.0, current_point.1 + 1)
    }
    else if next == 'B' {
        current_point = (current_point.0 + 1, current_point.1)
    }
    else {
        current_point = (current_point.0, current_point.1 - 1)
    }

    let mut last = pos_s;
    while current_point != pos_s {
        let origin = from(last, current_point);
        last = current_point;
        if origin == 'H' {
            if mat[current_point.0][current_point.1] == '|' {
                current_point = (current_point.0 + 1, current_point.1)
            }
            else if mat[current_point.0][current_point.1] == 'J' {
                current_point = (current_point.0, current_point.1 - 1)
            }
            else {
                current_point = (current_point.0, current_point.1 + 1)
            }
        }
        else if origin == 'D' {
            if mat[current_point.0][current_point.1] == '-' {
                current_point = (current_point.0, current_point.1 - 1)
            }
            else if mat[current_point.0][current_point.1] == 'F' {
                current_point = (current_point.0 + 1, current_point.1)
            }
            else {
                current_point = (current_point.0 - 1, current_point.1)
            }
        }
        else if origin == 'B' {
            if mat[current_point.0][current_point.1] == '|' {
                current_point = (current_point.0 - 1, current_point.1)
            }
            else if mat[current_point.0][current_point.1] == 'F' {
                current_point = (current_point.0, current_point.1 + 1)
            }
            else {
                current_point = (current_point.0, current_point.1 - 1)
            }
        }
        else {
            if mat[current_point.0][current_point.1] == '-' {
                current_point = (current_point.0, current_point.1 + 1)
            }
            else if mat[current_point.0][current_point.1] == 'J' {
                current_point = (current_point.0 - 1, current_point.1)
            }
            else {
                current_point = (current_point.0 + 1, current_point.1)
            }
        }
        score += 1;
    }

    println!("{:?}\n", mat);

    return (score + 1) /2;
}

fn part2(texte: &str) -> i64 {
    let mut score: i64 = 0;

    let lines = texte.lines();

    let mut mat: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let morceaux = line.chars();
        let mut tab : Vec<char> = Vec::new();

        for carac in morceaux {
            tab.push(carac);
            tab.push('-');
        }
        let supp = vec!['|'; tab.len()];
        mat.push(tab);
        mat.push(supp);
    }

    let mut mat_for_aire = vec![vec!['.';mat[0].len()];mat.len()];

    //Find the S 
    let hauteur = mat.len();
    let largeur = mat[0].len();

    let mut current_point = find_s(&mat, hauteur, largeur);
    print_x_at_point(&mut mat_for_aire, current_point);
    let pos_s = current_point;
    let next = look_around2(&mat, hauteur, largeur, pos_s.0 as isize, pos_s.1 as isize);
    
    if next == 'H' {
        current_point = (current_point.0 - 1, current_point.1)
    }
    else if next == 'D' {
        current_point = (current_point.0, current_point.1 + 1)
    }
    else if next == 'B' {
        current_point = (current_point.0 + 1, current_point.1)
    }
    else {
        current_point = (current_point.0, current_point.1 - 1)
    }

    print_x_at_point(&mut mat_for_aire, current_point);

    let mut last = pos_s;
    while current_point != pos_s {
        let origin = from(last, current_point);
        last = current_point;
        if origin == 'H' {
            if mat[current_point.0][current_point.1] == '|' {
                current_point = (current_point.0 + 1, current_point.1)
            }
            else if mat[current_point.0][current_point.1] == 'J' {
                current_point = (current_point.0, current_point.1 - 1)
            }
            else {
                current_point = (current_point.0, current_point.1 + 1)
            }
        }
        else if origin == 'D' {
            if mat[current_point.0][current_point.1] == '-' {
                current_point = (current_point.0, current_point.1 - 1)
            }
            else if mat[current_point.0][current_point.1] == 'F' {
                current_point = (current_point.0 + 1, current_point.1)
            }
            else {
                current_point = (current_point.0 - 1, current_point.1)
            }
        }
        else if origin == 'B' {
            if mat[current_point.0][current_point.1] == '|' {
                current_point = (current_point.0 - 1, current_point.1)
            }
            else if mat[current_point.0][current_point.1] == 'F' {
                current_point = (current_point.0, current_point.1 + 1)
            }
            else {
                current_point = (current_point.0, current_point.1 - 1)
            }
        }
        else {
            if mat[current_point.0][current_point.1] == '-' {
                current_point = (current_point.0, current_point.1 + 1)
            }
            else if mat[current_point.0][current_point.1] == 'J' {
                current_point = (current_point.0 - 1, current_point.1)
            }
            else {
                current_point = (current_point.0 + 1, current_point.1)
            }
        }
        print_x_at_point(&mut mat_for_aire, current_point);
    }

    for i in 0..hauteur {
        mat_for_aire[i].push('.');
        mat_for_aire[i].insert(0,'.');
    }
    mat_for_aire.push(vec!['.'; mat_for_aire[0].len()]);
    mat_for_aire.insert(0,vec!['.'; mat_for_aire[0].len()]);

    propage_o(&mut mat_for_aire, 0, 0);

    for i in 0..mat_for_aire.len() {
        println!("{:?}", mat_for_aire[i]);
    }

    for i in 0..hauteur{
        for j in 0..largeur{
            if mat_for_aire[i][j] == '.' && i % 2 != 0 && j % 2 != 0 {
                score += 1;
            }
        }
    }

    return score;
}

fn find_s(mat: &Vec<Vec<char>>, n: usize, m: usize) -> (usize, usize) {
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 'S' {
                return (i,j)
            }
        }
    }
    return (0,0)
}

fn look_around(mat: &Vec<Vec<char>>, n: usize, m: usize, i: isize, j: isize ) -> char {
    if i - 1 >= 0 && (mat[i as usize-1][j as usize ] == '|' || mat[i as usize -1][j as usize] == '7' || mat[i as usize -1][j as usize] == 'F') {
        return 'H'
    }
    else if j + 1 <= m as isize && (mat[i as usize][j as usize + 1] == '-' || mat[i as usize][j as usize + 1] == '7' || mat[i as usize][j as usize + 1] == 'J') {
        return 'D'
    }
    else if i + 1 <= n as isize && (mat[i as usize + 1][j as usize] == '|' || mat[i as usize + 1][j as usize] == 'J' || mat[i as usize + 1][j as usize] == 'L') {
        return 'B'
    }
    else {
        return 'G'
    }
}

fn look_around2(mat: &Vec<Vec<char>>, n: usize, m: usize, i: isize, j: isize ) -> char {
    if i - 2 >= 0 && (mat[i as usize-2][j as usize ] == '|' || mat[i as usize -2][j as usize] == '7' || mat[i as usize -2][j as usize] == 'F') {
        return 'H'
    }
    else if j + 2 <= m as isize && (mat[i as usize][j as usize + 2] == '-' || mat[i as usize][j as usize + 2] == '7' || mat[i as usize][j as usize + 2] == 'J') {
        return 'D'
    }
    else if i + 2 <= n as isize && (mat[i as usize + 2][j as usize] == '|' || mat[i as usize + 2][j as usize] == 'J' || mat[i as usize + 2][j as usize] == 'L') {
        return 'B'
    }
    else {
        return 'G'
    }
}

fn from(last : (usize,usize), current:(usize,usize)) -> char {
    if last.0 as isize - current.0 as isize == -1 {
        return 'H'
    }
    else if last.0 as isize - current.0 as isize == 1 {
        return 'B'
    }
    else if last.1 as isize - current.1 as isize == -1 {
        return 'G'
    }
    else {
        return 'D'
    }
}

fn print_x_at_point(mat: &mut Vec<Vec<char>>, point: (usize,usize)) -> () {
    mat[point.0][point.1] = 'X'
}

fn propage_o(mat: &mut Vec<Vec<char>>, i : usize, j : usize) -> () {
    mat[i][j] = 'O';
    if i as isize - 1 >= 0 && mat[i - 1][j] != 'X' && mat[i - 1][j] != 'O'{
        propage_o(mat, i-1, j)
    }
    if j as isize - 1 >= 0 && mat[i][j-1] != 'X' && mat[i][j-1] != 'O' {
        propage_o(mat, i, j-1)
    }
    if i + 1 < mat.len() && mat[i + 1][j] != 'X' && mat[i + 1][j] != 'O' {
        propage_o(mat, i+1, j)
    }
    if j + 1 < mat[0].len() && mat[i][j+1] != 'X' && mat[i][j+1] != 'O' {
        propage_o(mat, i, j+1)
    }
}
