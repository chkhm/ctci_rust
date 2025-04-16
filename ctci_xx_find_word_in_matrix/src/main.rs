
type Word = Vec<char>;
type Vec2d = Vec<Word>;
type Pos = [usize; 2];
type VecPos = Vec<Pos>;
type VecVecPos = Vec<VecPos>;


fn compute_position_list(c : char, matrix : & Vec2d) -> VecPos {
    let mut result = VecPos::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == c {
                result.push([i, j]);
            }
        }
    }
    result
}

fn compute_all_position_lists( word : &Word, matrix : & Vec2d) -> VecVecPos {
    let mut result = VecVecPos::new();

    for i in 0..word.len() {
        let c = word[i];
        result.push(compute_position_list(c, matrix));
    }
    result
}

fn are_neighbors(pos1 : &Pos, pos2 : &Pos)  -> bool {
    // if one of them is undefined we return true
    if pos1[0] == std::usize::MAX || pos2[0] == std::usize::MAX {
        return true;
    }
    let d1 = pos1[0].abs_diff(pos2[0]);
    let d2 = pos1[1].abs_diff(pos2[1]);

    // we want that both deltas <= 1 and they are not supposed to be the same
    (d1 < 2) && (d2 <2 ) && (d1 != d2)
}

fn search_word_in_matrix(word : &Word, matrix : &Vec2d) -> VecVecPos {
    
    let idx : usize = 0;
    let mut path = VecPos::new();
    let all_pos = compute_all_position_lists(&word, &matrix); 
    let mut result = VecVecPos::new();

    recursive_search(idx, &mut path, word, matrix, &all_pos, &mut result);

    result
}

fn recursive_search(idx : usize, path : &mut VecPos, word : &Word, matrix : & Vec2d, all_pos : &VecVecPos, result : &mut VecVecPos) {
    if idx >= word.len() {
        let p = path.clone();
        // print!("{:?}", &p);
        result.push(p);
        return;
    }
    for pos in all_pos[idx].iter() {
        let pos_last = match path.last() {
            Some(x) => x,
            None => &[std::usize::MAX, std::usize::MAX]
        } ;
        if !path.contains(pos) && are_neighbors(&pos, pos_last) {
            path.push(pos.clone());
            recursive_search(idx+1, path, word, matrix, all_pos, result);
            path.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let matrix = 
        vec![ vec![ 'a', 'b', 'c'],
              vec![ 'c', 'd', 'e'],
              vec![ 'd', 'e', 'f'],
              vec![ 'g', 'h', 'i'],
        ];

    let word = vec!['d', 'e', 'f'];

    //let all_pos = compute_all_position_lists(&word, &matrix);
    //println!("All Positions: {:?}", all_pos);
    //let mut path = VecPos::new();
    //let mut result = VecVecPos::new();
    //recursive_search(0, &mut path, &word, &matrix, &all_pos, &mut result);

    let result = search_word_in_matrix(&word, &matrix);

    println!("Len Result: {}", result.len());
    for r in result {
        println!("{:?} ", r);
    }
}
