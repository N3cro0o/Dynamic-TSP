use factorial::Factorial;

use super::Matrix;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TSP_Data{
    length: isize,
    target: usize,
    vertices: Vec<usize>
}

impl TSP_Data {
    pub fn is_the_same(&self, target: usize, vertices: Vec<usize>) -> bool {
        if target != self.target {
            return false;
        }
        if vertices.len() != self.vertices.len() {
            return false;
        }
        for v in 0..vertices.len(){
            if vertices[v] != self.vertices[v] {
                return false;
            }
        }
        true
    }
}

pub fn tsp_standard(matrix: &Matrix) -> Result<(Vec<usize>, usize), &'static str> {
    // Variables
    let max_factorial:u128 = (matrix.vertices as u128 - 1 ).factorial();
    let mut permutation_nr: u128 = 0;
    let m_vert = matrix.vertices;
    let mut test_vec: Vec<usize> = vec![0; m_vert]; // Store current path
    let mut test_answ_check = false;
    
    // Output
    let mut answ_vec: Vec<usize> = vec![0; m_vert];
    let mut answ_dist = usize::MAX;
    let mut answ: Result<(Vec<usize>, usize), &'static str> = Err("Can't find hamilton cycle");
    // Setup test_vec
    for i in 0 .. m_vert - 1 {
        test_vec[i] = i + 1;
    }

    // LST standard test
    'lst: loop {
        // Print percent
        if permutation_nr % 100_000_000 == 0 {
            let percent = permutation_nr as f64 / max_factorial as f64 * 100.0;
            println!("Current -> {percent}% done");
        }
        // Test to end the 'lst loop
        if *test_vec.get(0).unwrap() == m_vert {break 'lst;}

        // Prep
        let mut test_dist: usize = 0;
        let mut test_last: usize = 0;

        // Read permutation
        for i in test_vec.iter() {
            // Check if path exsts. If not, get new permutation
            if matrix.matrix[test_last][*i] == -1 {
                test_vec = std_update_vector(test_vec, m_vert);
                continue 'lst;
            }
            test_dist += matrix.matrix[test_last][*i] as usize;
            test_last = *i;
        }

        // Check answ
        if answ_dist > test_dist {
            test_answ_check = true;
            answ_dist = test_dist;
            answ_vec = test_vec.clone();
        }

        // Update permutation
        test_vec = std_update_vector(test_vec, m_vert);
        permutation_nr += 1;
    }
    if test_answ_check {
        answ = Ok((answ_vec, answ_dist));
    }
    answ
}

pub fn tsp_dyn(matrix: &Matrix) -> Option<usize> {
    let mut mem_vec: Vec<TSP_Data> = vec![];
    let mut perm_vec = vec![];
    let mut distance = usize::MAX;
    // Setup vertices vec
    for i in 1..matrix.vertices {
        perm_vec.push(i);
    }
    for i in 1..matrix.vertices {
        println!("Now checing vert {i}");
        let id = tsp_dyn_main(matrix, i, perm_vec.clone(), &mut mem_vec);
        let d = mem_vec[id].length + matrix.matrix[i][0];
        if (d as usize) < distance {distance = d as usize;}
    }
    if distance != usize::MAX {
        Some(distance)
    }
    else {
        None
    }
}

fn tsp_dyn_main(matrix: &Matrix,target: usize, perm_vec: Vec<usize>, mem_vec: &mut Vec<TSP_Data>) -> usize {
    // Setup
    let mut len = 0;
    let mut tsp_data = TSP_Data {
        target: target,
        vertices: perm_vec.clone(),
        length: 0
    };

    // Memorization
    for data in 0..mem_vec.len() {
        if mem_vec[data].is_the_same(target, perm_vec.clone()) {
            return data as usize;
        }
    }

    // Check if last
    if perm_vec.len() == 1{
        tsp_data.length = matrix.matrix[0][target];
        mem_vec.push(tsp_data);
        len = mem_vec.len() as usize - 1;
    }
    else {
        let mut distance = isize::MAX;
        // Create new vec
        let mut buff_vec = vec![];
        for j in perm_vec.iter() {
            // Skip chosen edge
            if *j == target {continue;}
            buff_vec.push(*j);
        }
        // Test
        for new_target in buff_vec.iter() {
            let id = tsp_dyn_main(matrix, *new_target, buff_vec.clone(), mem_vec);
            let d = mem_vec[id].length + matrix.matrix[*new_target][target];
            if d < distance {distance = d;}
        }
        tsp_data.length = distance;
        mem_vec.push(tsp_data);
        len = mem_vec.len() - 1;
    }
    len
}


// https://github.com/williamfiset
// Add infinite edge logic
pub fn tsp_dyn_new(m: &Matrix) -> isize {
    let size= m.vertices;
    let exp = 2_usize.checked_pow(size as u32).expect("Problem too big");
    let mut aux_mat: Vec<Vec<Option<isize>>> = vec![vec![None; exp]; size];
    
    // Store first distances; from ver 0 to each one
    for i in 1..size {
        let x = 1 << 0 | 1 << i;
        aux_mat[i][x] = Some(m.matrix[0][i]);
    }

    for i in 3 ..= size {
        println!("----- {size}, {i}");
        for sets in dyn_perms(size, i) {
            // checks if value x is part of the set (sets & (1 << x)) == 1, 
            // so (sets & (1 << x)) == 0 checks if x is not part of the set
            if (sets & (1 << 0)) == 0 {
                continue;
            }
            for next in 0..size {
                if next == 0 || (sets & (1 << next)) == 0 {
                    continue;
                }
                let mask = sets ^ (1 << next);
                let mut dist = isize::MAX;
                for end in 0..size {
                    if end == next || end == 0 || (sets & (1 << end)) == 0 {
                        continue;
                    }
                    let d = aux_mat[end][mask].unwrap() + m.matrix[end][next];
                    if d < dist {
                        dist = d;
                    }
                }
                aux_mat[next][sets] = Some(dist);
            }
        }
    }

    let finish = (1 << size) - 1;
    let mut min_dist = isize::MAX;

    for i in 1..size {
        let cost = aux_mat[i][finish].unwrap() + m.matrix[i][0];
        if cost < min_dist {
            min_dist = cost;
        }
    }

    min_dist
}

fn dyn_perms(size: usize, ones: usize) -> Vec<usize> {
     /* 
        TL;DR
        | - or
        & - and
        ^ - xor
        << - bit shift to the left (pow)
        >> - bit shift to the right (root)
      */
    let mut vec = vec![];
    // Smallest one:
    let mut target = (1 << ones) - 1;
    vec.push(target);

    'outer: loop {
        let mut p = 0;
        while p < size - 1 && ((target & (1 << (p + 1))) >> (p + 1)) >= ((target & (1 << (p))) >> (p)) {
            p += 1;
        }

        // Check if last
        if p == size - 1{
            break 'outer;
        }
        target |= 1 << (p + 1);

        for swap in 0.. (p + 1) {
            if (target & (1 << swap)) >> swap == 1 {
                target ^= 1 << swap;
                break;
            }
        }

        let mut end: usize = 0;
        while p > end {
            
            if ((target & (1 << end)) >> end) ^ ((target & (1 << p)) >> p) == 1 {
                target ^= 1 << end;
                target ^= 1 << p;
            }
            end += 1;
            p -= 1;
        }
        vec.push(target);
    }
    vec
}

pub fn print_all_permutations(vec_size: usize) -> usize {
    let mut vec: Vec<usize> = vec![];
    let mut repeats = 0;
    for i in 1..vec_size {
        vec.push(i);
    }
    vec.push(0);
    while vec[0] < vec_size {
        println!("{vec:?}");
        vec = std_update_vector(vec, vec_size);
        repeats += 1;
    }
    repeats
}

fn std_update_vector(vec: Vec<usize>, vertices: usize) -> Vec<usize> {
    let mut vec = vec;
    let mut pos = 0;
    let mut add = 1;

    // Find change position
    'outer: for i in 3 .. vertices {
        //println!("{}",vertices - i);
        let val = vec[vertices - i];
        //println!("{}", val + add);
        while val + add < vertices {
            if !crate::check_if_exist_in_vec(&vec[0 .. vertices - i], val + add) {
                pos = vertices - i;
                break 'outer;
            }
            add += 1;
        }
        add = 1;
    }

    // Change pos index
    vec[pos] += add;

    // Small QOL addition, print start vertices
    if pos == 0 {println!("Update --> now check ver nr. {}", vec[pos]);}

    // Update vec
    for i in pos + 1 .. vertices - 1 {
        let mut min = 1;
        loop {
            let mut b = false;
            for j in 0 .. i {
                if vec[j] == min {b = true; break;}
            }
            if !b {break;}
            min += 1;
        }
        vec[i] = min;
    }
    vec
}

/*fn std_update_vector(vec: Vec<usize>, vertices: usize) -> Result<Vec<usize>, &'static str> { // CHANGE IT. WRONG ORDER. CHECK FROM BEHIND
    // Variables
    let mut pos = 0; // Change position
    let mut vec = vec;

    // Check if valid
    match vec.get(0) {
        Some(x) => {
            if *x >= vertices {return Err("All permutations done");}
        }
        None => {return Err("0 size vector");}
    }

    // Check for position
    for i in vertices - 1 ..= 0 {


        if vec[i] + 1 >= vertices {
            pos = i;
            break;
        }
    }

    // Update vec
    vec[pos] += 1;
    for i in pos..vertices {
        // Find next valid value 
        let mut d = 0;
        'inner: loop {
            let mut repeat_check = false;
            for j in 0..i{
                if vec[j] == d {repeat_check = true;}
            }
            if !repeat_check {break 'inner;}
            d += 1;
        }
    }
    
    Ok(vec)
}*/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_vec(){
        let mut vec = vec![1, 2, 3, 4, 0];
        vec = std_update_vector(vec, 5);
        assert_eq!([1, 2, 4, 3, 0], *vec);
    }

    #[test]
    fn test_vec_overflow(){
        let mut vec = vec![4, 3, 2, 1, 0];
        vec = std_update_vector(vec, 5);
        assert_eq!([5, 1, 2, 3, 0], *vec);
    }

    #[test]
    fn test_vec_all_4(){
        let mut vec = vec![1, 2, 3, 0];
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [1, 3, 2, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [2, 1, 3, 0]);
        vec = std_update_vector(vec, 4); // ERROR!
        assert_eq!(vec, [2, 3, 1, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [3, 1, 2, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [3, 2, 1, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [4, 1, 2, 0]);
    }

    #[test]
    fn test_vec_slice(){
        let vec = vec![2, 1, 2, 3, 4];
        assert_eq!(true, crate::check_if_exist_in_vec(&vec[0.. 2], 2))
    }
}