use super::Matrix;

pub fn tsp_standard(matrix: &Matrix) -> Result<(Vec<usize>, usize), &'static str> {
    // Variables
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
    }
    if test_answ_check {
        answ = Ok((answ_vec, answ_dist));
    }
    answ
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
            if !check_if_exist_in_vec(&vec[0 .. vertices - i], val + add) {
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

fn check_if_exist_in_vec(vec: &[usize], val: usize) -> bool {
    for i in vec.iter(){
        if *i == val{
            return true;
        }
    }
    return false;
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
        assert_eq!(true, check_if_exist_in_vec(&vec[0.. 2], 2))
    }
}